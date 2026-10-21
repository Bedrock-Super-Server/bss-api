//! The [`Server`] handle and what hangs off it: worlds, entities, effects, the scheduler and services shared between plugins.

pub mod catalog;
pub mod effect;
pub mod entity;
pub mod scheduler;
mod services;
mod terrain;
pub mod world;

use crate::ffi::abi::{FfiStr, ItemStack};
use crate::ffi::call::HostFn;
use crate::ffi::host::{BlockEdit, HostApi};
use crate::player::Player;

const CHEST_SIZE: usize = 27;

/// Largest list [`read_all`] asks the host for.
const MAX_LIST: usize = 1 << 16;

/**
Reads a list from the host through `fill`, which writes at most `buffer.len()` items and returns how many it wrote.

The host cannot say how long a list is, so the buffer doubles until the host leaves part of it unused: a list longer than any fixed buffer, such as the names of every block, is never cut short.
*/
pub(crate) fn read_all<T: Copy + Default>(mut fill: impl FnMut(&mut [T]) -> usize) -> Vec<T> {
    let mut capacity = 64;
    loop {
        let mut buffer = vec![T::default(); capacity];
        let count = fill(&mut buffer).min(capacity);
        if count < capacity || capacity >= MAX_LIST {
            buffer.truncate(count);
            return buffer;
        }
        capacity *= 2;
    }
}

/// Copies bytes the host returned into an owned buffer.
pub(crate) fn ffi_bytes(value: FfiStr) -> Vec<u8> {
    // SAFETY: host buffers stay valid until the next host call from this thread
    // (see `HostApi`), and the bytes are copied before returning.
    unsafe { value.as_bytes() }.to_vec()
}

/// Copies a string the host returned into an owned one.
pub(crate) fn ffi_string(value: FfiStr) -> String {
    // SAFETY: host strings point into a buffer that stays valid until the next host call
    // from this thread (see `HostApi`), and the bytes are copied before returning.
    unsafe { value.as_str().map(|s| s.to_string()).unwrap_or_default() }
}

/// A plugin loaded on the server, as [`Server::plugins`] lists it.
#[derive(Clone, Debug)]
pub struct PluginInfo {
    /// Name of the plugin.
    pub name: String,
    /// Version of the plugin.
    pub version: String,
    /// SHA-256 of the library file, in hexadecimal.
    pub sha: String,
}

/// A dimension, which sets the sky, the fog and the height of a world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    /// The overworld.
    Overworld = 0,
    /// The Nether.
    Nether = 1,
    /// The End.
    End = 2,
}

/**
A handle to the server, given to every handler.

It is cheap to copy. Methods that take coordinates without a world name act on the default world; use [`Server::world`] for another one.

# Example

```
use bss_api::prelude::*;

fn status(server: &Server) -> String {
    format!(
        "{} players on {} ({} {}), {:.1} TPS",
        server.player_count(),
        server.name(),
        server.software_name(),
        server.software_version(),
        server.tps(),
    )
}
```
*/
#[derive(Clone, Copy)]
pub struct Server {
    host: &'static HostApi,
}

/// What a plugin takes of the server's time, from [`Server::plugin_usage`]: its own time, without
/// the calls into other plugins it makes.
#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
pub struct PluginUsage {
    /// Name of the plugin.
    pub name: String,
    /// Calls the server made into the plugin since it was loaded.
    pub calls: u64,
    /// Milliseconds the plugin ran since it was loaded.
    pub busy_ms: f64,
    /// Its longest call, in microseconds.
    pub max_us: f64,
    /// Percentage of the last second the plugin ran, across all threads.
    pub last_second_percent: f64,
    /// Percentage of the time since it was loaded the plugin ran.
    pub average_percent: f64,
}

/// A packet the server knows, from [`Server::packets`].
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
pub struct PacketInfo {
    /// Number of the packet in the protocol.
    pub id: u32,
    /// Name of the packet, as [`Player::send_packet`](crate::player::Player::send_packet) takes it.
    pub name: String,
}

impl Server {
    /**
    # Safety

    `host` must point at the server's `HostApi`, which lives for the whole process.
    */
    pub(crate) unsafe fn from_raw(host: *const HostApi) -> Self {
        // SAFETY: guaranteed by the caller.
        Self::from_host(unsafe { &*host })
    }

    pub(crate) fn from_host(host: &'static HostApi) -> Self {
        Self { host }
    }

    pub(crate) fn host(&self) -> &'static HostApi {
        self.host
    }

    pub(crate) fn player_from_handle(&self, handle: u64) -> Option<Player> {
        (handle != 0).then(|| Player::new(handle, self.host))
    }

    /// Sends `message` to the chat of every player. `§` color codes work.
    pub fn broadcast(&self, message: &str) {
        host_call!(self.host, broadcast(message.as_ptr(), message.len()));
    }

    /**
    Sends an HTTP request from a thread of the server and calls `on_answer` with its answer, on
    a thread of the server too.

    `request` is `{ "uri", "method", "headers": [{ "key", "value" }], "body", "timeout" }`, the
    timeout in seconds; only `uri` is required, and the method defaults to `GET`. The answer is
    `{ "status", "headers": [{ "key", "value" }], "body" }`, or `{ "error" }`.

    Returns `false`, and drops `on_answer`, when the request is not sent: on a server too old to
    send it, or one whose `plugins.toml` does not grant this plugin `http`.
    */
    pub fn http_request(
        &self,
        request: &serde_json::Value,
        on_answer: impl FnOnce(serde_json::Value) + Send + 'static,
    ) -> bool {
        let Some(f) = self.host.http_request else {
            return false;
        };
        let handler: Box<dyn FnOnce(&str) + Send> = Box::new(move |answer: &str| {
            on_answer(
                serde_json::from_str(answer)
                    .unwrap_or_else(|error| serde_json::json!({ "error": error.to_string() })),
            );
        });
        let user_data = crate::registry::callbacks::box_form(handler);
        let request = request.to_string();
        let sent = f.call((
            FfiStr::new(&request),
            crate::ffi::abi::HttpCallback::Some(crate::registry::callbacks::answer_form),
            user_data,
        ));
        if !sent {
            // SAFETY: the server did not take the handler: it is freed here, once.
            drop(unsafe { crate::registry::callbacks::unbox_form(user_data) });
        }
        sent
    }

    /// Sends the packet `name`, with `fields` as JSON, to every player; the server builds it once
    /// for each protocol version they speak. See [`Player::send_packet`](crate::player::Player::send_packet).
    pub fn broadcast_packet(&self, name: &str, fields: &serde_json::Value) -> Result<(), String> {
        self.broadcast_packet_in("", name, fields)
    }

    /// Sends the packet `name`, with `fields` as JSON, to every player in the world named `world`.
    pub fn broadcast_packet_to_world(
        &self,
        world: &str,
        name: &str,
        fields: &serde_json::Value,
    ) -> Result<(), String> {
        if world.is_empty() {
            return Err("name a world, or use broadcast_packet for every player".to_string());
        }
        self.broadcast_packet_in(world, name, fields)
    }

    fn broadcast_packet_in(
        &self,
        world: &str,
        name: &str,
        fields: &serde_json::Value,
    ) -> Result<(), String> {
        let fields = fields.to_string();
        match host_call!(
            self.host,
            packet_broadcast(FfiStr::new(world), FfiStr::new(name), FfiStr::new(&fields))
        ) {
            Some(answer) => {
                let error = ffi_string(answer);
                if error.is_empty() { Ok(()) } else { Err(error) }
            }
            None => Err("the server does not send packets built from JSON".to_string()),
        }
    }

    /// What each loaded plugin takes of the server's time, the busiest first.
    pub fn plugin_usage(&self) -> Vec<PluginUsage> {
        host_call!(self.host, plugin_usage())
            .map(ffi_string)
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }

    /// The name and number of every packet the server knows, for
    /// [`Player::send_packet`](crate::player::Player::send_packet) and [`PacketEvent`](crate::event::PacketEvent).
    pub fn packets(&self) -> Vec<PacketInfo> {
        host_call!(self.host, packet_list())
            .map(ffi_string)
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }

    /// Runs `command`, without its leading `/`, as the console. Returns `false` when the command is empty or the server is not ready.
    pub fn run_command(&self, command: &str) -> bool {
        host_call!(
            self.host,
            dispatch_command(FfiStr::new("CONSOLE"), FfiStr::new(command))
        )
        .unwrap_or(false)
    }

    /// Runs `command` as if `player` had typed it, with their permissions. Returns `false` when no such player is online.
    pub fn run_command_as(&self, player: &str, command: &str) -> bool {
        host_call!(
            self.host,
            dispatch_command(FfiStr::new(player), FfiStr::new(command))
        )
        .unwrap_or(false)
    }

    /// Bans a player, by name or XUID, or an IP address for `duration_secs` seconds; `0` bans for ever. The ban follows the player's XUID, recorded at once for an online player and at their next login otherwise. Banning an online player also bans their IP, unless it is a proxy or a trusted address, and kicks them with `reason`.
    pub fn ban(&self, target: &str, duration_secs: u64, reason: &str) {
        host_call!(
            self.host,
            server_ban(FfiStr::new(target), duration_secs, FfiStr::new(reason))
        );
    }

    /// Lifts the ban on a player, by name or XUID, or on an IP address. Returns `false` when it was not banned.
    pub fn unban(&self, target: &str) -> bool {
        host_call!(self.host, server_unban(FfiStr::new(target))).unwrap_or(false)
    }

    /// Adds a player, by name or XUID, to the whitelist; the entry follows their XUID. Returns `false` when they were already there.
    pub fn whitelist_add(&self, name: &str) -> bool {
        host_call!(self.host, server_whitelist_add(FfiStr::new(name))).unwrap_or(false)
    }

    /// Removes a player, by name or XUID, from the whitelist. Returns `false` when they were not there.
    pub fn whitelist_remove(&self, name: &str) -> bool {
        host_call!(self.host, server_whitelist_remove(FfiStr::new(name))).unwrap_or(false)
    }

    /// Turns the whitelist on or off. Returns `false` when it already was.
    pub fn whitelist_enabled(&self, enabled: bool) -> bool {
        host_call!(self.host, server_whitelist_set(enabled)).unwrap_or(false)
    }

    /// Makes a player, by name or XUID, an operator; the entry follows their XUID. Returns `false` when they already were.
    pub fn op(&self, name: &str) -> bool {
        host_call!(self.host, server_op(FfiStr::new(name))).unwrap_or(false)
    }

    /// Removes a player, by name or XUID, from the operators. Returns `false` when they were not one.
    pub fn deop(&self, name: &str) -> bool {
        host_call!(self.host, server_deop(FfiStr::new(name))).unwrap_or(false)
    }

    /// Returns the name of the biome the generator places at `(x, z)` in the default world, such as `plains`.
    pub fn biome_at(&self, x: i32, z: i32) -> String {
        host_call!(self.host, world_biome(x, z))
            .map(crate::server::ffi_string)
            .unwrap_or_default()
    }

    /// Sets a boolean game rule, such as `doDaylightCycle`, and tells every player.
    pub fn set_gamerule(&self, name: &str, value: bool) {
        host_call!(self.host, set_gamerule(FfiStr::new(name), value));
    }

    /// Returns a boolean game rule, or `default` when it was never set.
    pub fn gamerule(&self, name: &str, default: bool) -> bool {
        host_call!(self.host, get_gamerule(FfiStr::new(name), default)).unwrap_or(default)
    }

    /// Loads a chunk of the default world, generating it if needed. Returns `true` when it was generated for the first time.
    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        host_call!(self.host, world_load_chunk(chunk_x, chunk_z)).unwrap_or(false)
    }

    /// Loads the world stored in `directory` as `name`, creating it if it does not exist. A bare directory name is looked up under `worlds/`; a path is used as it is. Loading a world that is already loaded succeeds. Returns `false` when the world cannot be opened.
    pub fn load_world(&self, name: &str, directory: &str, dimension: Dimension) -> bool {
        host_call!(
            self.host,
            world_load(FfiStr::new(name), FfiStr::new(directory), dimension as i32)
        )
        .unwrap_or(false)
    }

    /// Returns the names of the loaded worlds.
    pub fn worlds(&self) -> Vec<String> {
        let Some(f) = self.host.world_list else {
            return Vec::new();
        };
        let mut names = Vec::new();
        read_all(|buffer: &mut [FfiStr]| {
            let count = f.call((buffer.as_mut_ptr(), buffer.len() as u32)) as usize;
            // Host strings only live until the next host call: copy them now.
            names = buffer
                .iter()
                .take(count)
                .map(|name| ffi_string(*name))
                .collect();
            count
        });
        names
    }

    /// Returns the name of the default world, where players spawn.
    pub fn default_world(&self) -> String {
        host_call!(self.host, world_default())
            .map(crate::server::ffi_string)
            .unwrap_or_default()
    }

    /// Runs `task` at the start of the next tick, on the thread that runs the world.
    pub fn run_on_main(&self, task: impl Fn(&Server) + Send + Sync + 'static) {
        self.schedule(0, task);
    }

    /// Sends an encoded packet to every player who sees the chunk at `(chunk_x, chunk_z)` of the default world.
    pub fn broadcast_chunk_packet(&self, chunk_x: i32, chunk_z: i32, data: &[u8]) {
        host_call!(
            self.host,
            broadcast_chunk(chunk_x, chunk_z, data.as_ptr(), data.len())
        );
    }

    /// Returns the online player named `name`, ignoring case.
    pub fn player(&self, name: &str) -> Option<Player> {
        let handle = host_call!(self.host, player_by_name(FfiStr::new(name))).unwrap_or(0);
        self.player_from_handle(handle)
    }

    /// Returns the online player whose runtime id is `entity_id`.
    pub fn entity_player(&self, entity_id: u64) -> Option<Player> {
        let handle = host_call!(self.host, player_by_entity_id(entity_id)).unwrap_or(0);
        self.player_from_handle(handle)
    }

    /// Returns whether `entity_id` is the runtime id of an online player.
    pub fn is_player(&self, entity_id: u64) -> bool {
        self.entity_player(entity_id).is_some()
    }

    /**
    Returns every online player.

    ```
    # use bss_api::prelude::*;
    fn heal_everyone(server: &Server) {
        for player in server.players() {
            player.set_health(20.0);
        }
    }
    ```
    */
    pub fn players(&self) -> Vec<Player> {
        let Some(all) = self.host.player_all else {
            return Vec::new();
        };
        let mut capacity = (self.player_count() as usize).saturating_add(8);
        loop {
            let mut buf = vec![0u64; capacity];
            let count = all.call((buf.as_mut_ptr(), capacity as u32)) as usize;
            if count < capacity {
                buf.truncate(count);
                return buf
                    .into_iter()
                    .map(|handle| Player::new(handle, self.host))
                    .collect();
            }
            capacity = capacity.saturating_mul(2);
        }
    }

    /// Returns the number of online players.
    pub fn player_count(&self) -> u32 {
        host_call!(self.host, player_count()).unwrap_or(0)
    }

    /// Returns the numeric id of the item named `name`, such as `minecraft:diamond`.
    pub fn item_id(&self, name: &str) -> Option<i16> {
        let id = host_call!(self.host, item_id(FfiStr::new(name))).unwrap_or(i16::MAX);
        (id != i16::MAX).then_some(id)
    }

    /// Returns the durability of a tool, weapon or armor piece, or `None` for an item that does not wear out.
    pub fn max_durability(&self, item_id: i16) -> Option<u16> {
        let max = host_call!(self.host, item_max_durability(item_id)).unwrap_or(0);
        (max > 0).then_some(max)
    }

    /// Returns the name of an item, such as `minecraft:diamond`, or an empty string for an unknown id.
    pub fn item_name(&self, item_id: i16) -> String {
        host_call!(self.host, item_name(item_id))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the runtime id of the default state of the block named `name`, such as `minecraft:stone`, or `0` (air) for an unknown name.
    pub fn block_runtime_id(&self, name: &str) -> u32 {
        host_call!(self.host, block_runtime_id(FfiStr::new(name))).unwrap_or(0)
    }

    /// Returns the name of a block, such as `minecraft:stone`.
    pub fn block_name(&self, runtime_id: u32) -> String {
        host_call!(self.host, block_name(runtime_id))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns how fast `item_id` mines blocks of the `tool` family; a bare hand is `1.0`.
    pub fn mining_speed(&self, item_id: i16, tool: crate::content::ToolType) -> f32 {
        let tool = match tool {
            crate::content::ToolType::Pickaxe => 0,
            crate::content::ToolType::Axe => 1,
            crate::content::ToolType::Shovel => 2,
            crate::content::ToolType::Hoe => 3,
            crate::content::ToolType::Sword => 4,
            crate::content::ToolType::Shears => 5,
        };
        host_call!(self.host, item_mining_speed(item_id, tool)).unwrap_or(0.0)
    }

    /// Returns a handle to the world named `name`. The handle does not check that the world is loaded: its methods do nothing on an unknown world.
    pub fn world(&self, name: &str) -> world::World {
        world::World::new(self.host, name.to_owned())
    }

    /// Returns the time of day of the default world, in ticks: `0` is sunrise, `6000` noon, `13000` night.
    pub fn time(&self) -> i64 {
        host_call!(self.host, world_time()).unwrap_or(0)
    }

    /// Sets the time of day of the default world, in ticks.
    pub fn set_time(&self, time: i64) {
        host_call!(self.host, world_set_time(time));
    }

    /// Returns the seed of the default world.
    pub fn seed(&self) -> u64 {
        host_call!(self.host, world_seed()).unwrap_or(0)
    }

    /// Returns the spawn point of the default world.
    pub fn world_spawn(&self) -> Option<(i32, i32, i32)> {
        let f = self.host.world_get_spawn?;
        let (mut x, mut y, mut z) = (0, 0, 0);
        if f.call((&raw mut x, &raw mut y, &raw mut z)) {
            Some((x, y, z))
        } else {
            None
        }
    }

    /// Returns the number of chunks loaded in the default world.
    pub fn chunk_count(&self) -> u32 {
        host_call!(self.host, world_chunk_count()).unwrap_or(0)
    }

    /// Returns the maximum number of players set in `server.toml`.
    pub fn max_players(&self) -> u32 {
        host_call!(self.host, config_max_players()).unwrap_or(0)
    }

    /// Returns the server name shown in the server list.
    pub fn name(&self) -> String {
        host_call!(self.host, config_server_name())
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the ticks per second, averaged over the last 20 ticks; `20.0` is full speed.
    pub fn tps(&self) -> f64 {
        host_call!(self.host, config_tps()).unwrap_or(0.0)
    }

    /// Returns the name of the server software: `Bedrock Super Server`.
    pub fn software_name(&self) -> String {
        host_call!(self.host, software_name())
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the version of the server software.
    pub fn software_version(&self) -> String {
        host_call!(self.host, software_version())
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the commit the server was built from.
    pub fn software_sha(&self) -> String {
        host_call!(self.host, software_sha())
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the loaded plugins.
    pub fn plugins(&self) -> Vec<PluginInfo> {
        let Some(f) = self.host.plugin_list else {
            return Vec::new();
        };
        let mut names = [FfiStr::default(); 64];
        let mut versions = [FfiStr::default(); 64];
        let mut shas = [FfiStr::default(); 64];
        let count = (f.call((
            names.as_mut_ptr(),
            versions.as_mut_ptr(),
            shas.as_mut_ptr(),
            names.len() as u32,
        )) as usize)
            .min(names.len());
        (0..count)
            .map(|i| PluginInfo {
                name: ffi_string(names[i]),
                version: ffi_string(versions[i]),
                sha: ffi_string(shas[i]),
            })
            .collect()
    }

    /// Returns the Minecraft version the server speaks, such as `1.26.52`.
    pub fn minecraft_version(&self) -> String {
        host_call!(self.host, minecraft_version())
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the protocol number of that version.
    pub fn minecraft_protocol(&self) -> String {
        host_call!(self.host, minecraft_protocol())
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Reserves a runtime id for an entity the plugin creates.
    pub fn next_entity_id(&self) -> u64 {
        host_call!(self.host, next_entity_id()).unwrap_or(0)
    }

    /// Returns the runtime id of the block at `(x, y, z)` in the default world.
    pub fn block_at(&self, x: i32, y: i32, z: i32) -> u32 {
        host_call!(self.host, world_block_at(x, y, z)).unwrap_or(0)
    }

    /**
    Places the block `runtime_id` at `(x, y, z)` in the default world and shows it to players.

    ```
    # use bss_api::prelude::*;
    fn pillar(server: &Server, x: i32, z: i32) {
        let gold = server.block_runtime_id("minecraft:gold_block");
        for y in 64..70 {
            server.set_block(x, y, z, gold);
        }
    }
    ```
    */
    pub fn set_block(&self, x: i32, y: i32, z: i32, runtime_id: u32) {
        host_call!(self.host, world_set_block(x, y, z, runtime_id));
    }

    /// Drops `item` at `x`, `y`, `z` of the world named `world` (empty: the default world), as the
    /// server drops items, for players to pick up. Returns whether it was dropped.
    pub fn drop_item(
        &self,
        world: &str,
        x: f32,
        y: f32,
        z: f32,
        item: &crate::player::Item,
    ) -> bool {
        let Some(f) = self.host.world_drop_item else {
            return false;
        };
        f.call((
            FfiStr::new(world),
            x,
            y,
            z,
            item.item_id,
            item.block_runtime_id,
            item.count,
            item.damage,
            FfiStr::new(&item.name),
            FfiStr::new(&item.lore_json()),
            FfiStr::new(&item.ench_json()),
            item.unbreakable,
        ))
    }

    /// Places many blocks in the default world at once, as `(x, y, z, runtime_id)`, and returns how many changed. Much faster than calling [`set_block`](Self::set_block) in a loop.
    pub fn set_blocks(&self, edits: &[(i32, i32, i32, u32)]) -> u32 {
        let Some(f) = self.host.world_set_blocks else {
            return 0;
        };
        let edits: Vec<BlockEdit> = edits
            .iter()
            .map(|&(x, y, z, runtime_id)| BlockEdit {
                x,
                y,
                z,
                runtime_id,
            })
            .collect();
        f.call((edits.as_ptr(), edits.len() as u32))
    }

    /// Returns the runtime ids of every state of the block named `name`, such as the eight growth stages of `minecraft:wheat`.
    pub fn block_variants(&self, name: &str) -> Vec<u32> {
        let Some(f) = self.host.block_variants else {
            return Vec::new();
        };
        read_all(|buffer: &mut [u32]| {
            f.call((FfiStr::new(name), buffer.as_mut_ptr(), buffer.len() as u32)) as usize
        })
    }

    /// Returns the states of a block, as `(name, value)`, such as `("growth", "7")`.
    pub fn block_states(&self, runtime_id: u32) -> Vec<(String, String)> {
        let Some(f) = self.host.block_states else {
            return Vec::new();
        };
        let mut keys = [FfiStr::default(); 32];
        let mut values = [FfiStr::default(); 32];
        let count = (f.call((
            runtime_id,
            keys.as_mut_ptr(),
            values.as_mut_ptr(),
            keys.len() as u32,
        )) as usize)
            .min(keys.len());
        (0..count)
            .map(|i| (ffi_string(keys[i]), ffi_string(values[i])))
            .collect()
    }

    /// Returns the names of every vanilla block, sorted, without the `minecraft:` prefix.
    pub fn block_names(&self) -> Vec<String> {
        let Some(f) = self.host.block_names_all else {
            return Vec::new();
        };
        let mut names = Vec::new();
        read_all(|buffer: &mut [FfiStr]| {
            let count = f.call((buffer.as_mut_ptr(), buffer.len() as u32)) as usize;
            names = buffer
                .iter()
                .take(count)
                .map(|name| ffi_string(*name))
                .collect();
            count
        });
        names
    }

    /// Returns every permission the server knows, including those of plugin commands.
    pub fn registered_permissions(&self) -> Vec<String> {
        let Some(f) = self.host.permission_all else {
            return Vec::new();
        };
        let mut permissions = Vec::new();
        read_all(|buffer: &mut [FfiStr]| {
            let count = f.call((buffer.as_mut_ptr(), buffer.len() as u32)) as usize;
            permissions = buffer
                .iter()
                .take(count)
                .map(|permission| ffi_string(*permission))
                .collect();
            count
        });
        permissions
    }

    /// Returns the 27 slots of the chest at `(x, y, z)` in the default world, or an empty list when there is no chest there.
    pub fn chest_at(&self, x: i32, y: i32, z: i32) -> Vec<ItemStack> {
        let Some(f) = self.host.world_get_chest else {
            return Vec::new();
        };
        let mut buffer = [ItemStack::default(); CHEST_SIZE];
        let count =
            (f.call((x, y, z, buffer.as_mut_ptr(), buffer.len() as u32)) as usize).min(CHEST_SIZE);
        buffer[..count].to_vec()
    }

    /// Replaces the contents of the chest at `(x, y, z)` in the default world; slots past `slots` are emptied.
    pub fn set_chest(&self, x: i32, y: i32, z: i32, slots: &[ItemStack]) {
        host_call!(
            self.host,
            world_set_chest(x, y, z, slots.as_ptr(), slots.len() as u32)
        );
    }

    /// Writes a line to the item transaction log, the server's audit trail of where items come from and go: `action` such as `GIVE`, the `player`, the item and its unique id, and free text in `extra`.
    pub fn audit_item(
        &self,
        action: &str,
        player: &str,
        gamemode: i32,
        uid: u64,
        item_id: i16,
        count: u8,
        extra: &str,
    ) {
        host_call!(
            self.host,
            audit_item(
                FfiStr::new(action),
                FfiStr::new(player),
                gamemode,
                uid,
                item_id,
                count,
                FfiStr::new(extra)
            )
        );
    }
}

#[cfg(test)]
mod tests {
    use super::read_all;

    fn host_list(len: usize) -> impl FnMut(&mut [u32]) -> usize {
        move |buffer: &mut [u32]| {
            let count = len.min(buffer.len());
            for (index, slot) in buffer.iter_mut().take(count).enumerate() {
                *slot = index as u32;
            }
            count
        }
    }

    #[test]
    fn a_list_longer_than_the_first_buffer_is_read_in_full() {
        let all = read_all(host_list(300));
        assert_eq!(all.len(), 300);
        assert_eq!(all[299], 299);
    }

    #[test]
    fn a_list_as_long_as_a_buffer_is_read_in_full() {
        assert_eq!(read_all(host_list(64)).len(), 64);
        assert_eq!(read_all(host_list(128)).len(), 128);
    }

    #[test]
    fn an_empty_list_reads_as_empty() {
        assert!(read_all(host_list(0)).is_empty());
    }
}
