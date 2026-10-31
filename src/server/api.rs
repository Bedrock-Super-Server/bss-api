/*!
The functions of the server, grouped by what they work on.

[`Server`] hands out one handle for each group: [`bans`](Server::bans), [`access`](Server::access),
[`scheduler`](Server::scheduler), [`worlds`](Server::worlds), [`blocks`](Server::blocks),
[`items`](Server::items) and [`entities`](Server::entities). A handle is as cheap to copy as the
server, and the methods of one group are found together instead of among a hundred.

The calls that can fail answer with an [`ApiResult`](crate::error::ApiResult) that says why: the
plugin lacks a grant, the server is too old, or what the call names does not exist.

```
use bss_api::prelude::*;

fn tidy(server: &Server, name: &str) -> ApiResult<()> {
    server.bans().ban(name, None, "griefing")?;
    server.entities().remove(42)?;
    server.worlds().load("arena", "arena", Dimension::Overworld)?;
    Ok(())
}
```
*/

#![allow(deprecated)]

use std::time::Duration;

use crate::content::ToolType;
use crate::error::{ApiError, ApiResult};
use crate::ffi::abi::{FfiStr, ItemStack};
use crate::player::Item;
use crate::server::Server;
use crate::server::effect::Effect;
use crate::server::entity::{ActorFlag, Entity};
use crate::server::scheduler::TaskId;
use crate::server::world::World;
use crate::types::{BlockId, Dimension, ItemId, Location};

/// How long a tick lasts: the server runs twenty a second.
const TICK: Duration = Duration::from_millis(50);

/// What a call that names an entity answers when there is none.
fn found(done: bool) -> ApiResult<()> {
    if done {
        Ok(())
    } else {
        Err(ApiError::NotFound)
    }
}

impl Server {
    /// Bans and unbans players and addresses on this server.
    pub fn bans(&self) -> Bans {
        Bans { server: *self }
    }

    /// The whitelist and the operators.
    pub fn access(&self) -> Access {
        Access { server: *self }
    }

    /// Runs code later or repeatedly.
    pub fn scheduler(&self) -> Scheduler {
        Scheduler { server: *self }
    }

    /// The worlds, and what concerns the default one.
    pub fn worlds(&self) -> Worlds {
        Worlds { server: *self }
    }

    /// Reading and placing blocks of the default world, chests, and the blocks of the game by name.
    pub fn blocks(&self) -> Blocks {
        Blocks { server: *self }
    }

    /// The items of the game, by name and by id.
    pub fn items(&self) -> Items {
        Items { server: *self }
    }

    /// The entities plugins add, and the effects entities and players carry.
    pub fn entities(&self) -> Entities {
        Entities { server: *self }
    }
}

/// Bans, from [`Server::bans`].
#[derive(Clone, Copy)]
pub struct Bans {
    server: Server,
}

impl Bans {
    /**
    Bans a player, by name or XUID, or an IP address, for `duration`, or for ever with `None`. The ban follows the
    player's XUID, recorded at once for an online player and at their next login otherwise. Banning an online player
    also bans their IP, unless it is a trusted address, and kicks them with `reason`.

    Needs the `moderation` grant.
    */
    pub fn ban(&self, target: &str, duration: Option<Duration>, reason: &str) -> ApiResult<()> {
        let seconds = duration.map_or(0, |duration| duration.as_secs().max(1));
        host_try!(
            self.server.host(),
            server_ban(FfiStr::new(target), seconds, FfiStr::new(reason))
        )
        .map(|_| ())
    }

    /// Lifts the ban on a player, by name or XUID, or on an IP address. Answers whether it was banned. Needs the `moderation` grant.
    pub fn unban(&self, target: &str) -> ApiResult<bool> {
        host_try!(self.server.host(), server_unban(FfiStr::new(target)))
    }
}

/// The whitelist and the operators, from [`Server::access`].
#[derive(Clone, Copy)]
pub struct Access {
    server: Server,
}

impl Access {
    /// Adds a player, by name or XUID, to the whitelist; the entry follows their XUID. Answers whether they were not there already. Needs the `moderation` grant.
    pub fn whitelist_add(&self, name: &str) -> ApiResult<bool> {
        host_try!(self.server.host(), server_whitelist_add(FfiStr::new(name)))
    }

    /// Removes a player, by name or XUID, from the whitelist. Answers whether they were there. Needs the `moderation` grant.
    pub fn whitelist_remove(&self, name: &str) -> ApiResult<bool> {
        host_try!(
            self.server.host(),
            server_whitelist_remove(FfiStr::new(name))
        )
    }

    /// Turns the whitelist on or off. Answers whether that changed anything. Needs the `moderation` grant.
    pub fn set_whitelist(&self, enabled: bool) -> ApiResult<bool> {
        host_try!(self.server.host(), server_whitelist_set(enabled))
    }

    /// Makes a player, by name or XUID, an operator; the entry follows their XUID. Answers whether they were not one already. Needs the `operators` grant.
    pub fn op(&self, name: &str) -> ApiResult<bool> {
        host_try!(self.server.host(), server_op(FfiStr::new(name)))
    }

    /// Removes a player, by name or XUID, from the operators. Answers whether they were one. Needs the `operators` grant.
    pub fn deop(&self, name: &str) -> ApiResult<bool> {
        host_try!(self.server.host(), server_deop(FfiStr::new(name)))
    }
}

/// Tasks, from [`Server::scheduler`]. A task runs on the thread that runs the world.
#[derive(Clone, Copy)]
pub struct Scheduler {
    server: Server,
}

impl Scheduler {
    /// Runs `task` once, `delay_ticks` ticks from now (at least one). Twenty ticks make a second.
    pub fn schedule(
        &self,
        delay_ticks: u64,
        task: impl Fn(&Server) + Send + Sync + 'static,
    ) -> TaskId {
        self.server.schedule(delay_ticks, task)
    }

    /// Runs `task` every `interval_ticks` ticks, the first time that many ticks from now.
    pub fn schedule_repeating(
        &self,
        interval_ticks: u64,
        task: impl Fn(&Server) + Send + Sync + 'static,
    ) -> TaskId {
        self.server.schedule_repeating(interval_ticks, task)
    }

    /// Runs `task` once after `delay`, to the nearest tick.
    pub fn after(&self, delay: Duration, task: impl Fn(&Server) + Send + Sync + 'static) -> TaskId {
        self.schedule((delay.as_millis() / TICK.as_millis()) as u64, task)
    }

    /// Runs `task` every `interval`, to the nearest tick.
    pub fn every(
        &self,
        interval: Duration,
        task: impl Fn(&Server) + Send + Sync + 'static,
    ) -> TaskId {
        self.schedule_repeating((interval.as_millis() / TICK.as_millis()) as u64, task)
    }

    /// Runs `task` at the start of the next tick.
    pub fn next_tick(&self, task: impl Fn(&Server) + Send + Sync + 'static) {
        self.server.run_on_main(task);
    }

    /// Cancels a task. Answers `false` when it already ran or was cancelled.
    pub fn cancel(&self, id: TaskId) -> bool {
        self.server.cancel_task(id)
    }
}

/// The worlds, from [`Server::worlds`].
#[derive(Clone, Copy)]
pub struct Worlds {
    server: Server,
}

impl Worlds {
    /// The names of the loaded worlds.
    pub fn list(&self) -> Vec<String> {
        self.server.world_names()
    }

    /// A handle on the world named `name`. It does not check that the world is loaded: its methods do nothing on an unknown world.
    pub fn get(&self, name: &str) -> World {
        self.server.world(name)
    }

    /// The name of the default world, where players spawn.
    pub fn default_name(&self) -> String {
        self.server.default_world()
    }

    /// A handle on the default world.
    pub fn default(&self) -> World {
        self.server.world(&self.server.default_world())
    }

    /**
    Loads the world stored in `directory` as `name`, creating it if it does not exist. A bare directory name is looked up
    under `worlds/`; a path is used as it is. Loading a world that is already loaded succeeds.

    Needs the `worlds` grant, for a folder under `worlds/` only. Answers [`ApiError::Refused`] when the world cannot be opened.
    */
    pub fn load(&self, name: &str, directory: &str, dimension: Dimension) -> ApiResult<()> {
        let opened = host_try!(
            self.server.host(),
            world_load(FfiStr::new(name), FfiStr::new(directory), dimension.id())
        )?;
        if opened {
            Ok(())
        } else {
            Err(ApiError::Refused)
        }
    }

    /// The time of day of the default world, in ticks: `0` is sunrise, `6000` noon, `13000` night.
    pub fn time(&self) -> i64 {
        self.server.time()
    }

    /// Sets the time of day of the default world, in ticks.
    pub fn set_time(&self, time: i64) {
        self.server.set_time(time);
    }

    /// The seed of the default world.
    pub fn seed(&self) -> u64 {
        self.server.seed()
    }

    /// The spawn point of the default world.
    pub fn spawn(&self) -> Option<(i32, i32, i32)> {
        self.server.world_spawn()
    }

    /// How many chunks the default world has loaded.
    pub fn chunk_count(&self) -> u32 {
        self.server.chunk_count()
    }

    /// Loads a chunk of the default world, generating it if needed. Answers whether it was generated for the first time.
    pub fn load_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.server.load_chunk(chunk_x, chunk_z)
    }

    /// The name of the biome the generator places at `(x, z)` in the default world, such as `plains`.
    pub fn biome_at(&self, x: i32, z: i32) -> String {
        self.server.biome_at(x, z)
    }

    /// A boolean game rule, such as `doDaylightCycle`, or `default` when it was never set.
    pub fn gamerule(&self, name: &str, default: bool) -> bool {
        self.server.gamerule(name, default)
    }

    /// Sets a boolean game rule and tells every player.
    pub fn set_gamerule(&self, name: &str, value: bool) {
        self.server.set_gamerule(name, value);
    }
}

/// Blocks, from [`Server::blocks`]. Coordinates are those of the default world.
#[derive(Clone, Copy)]
pub struct Blocks {
    server: Server,
}

impl Blocks {
    /// The block at `(x, y, z)`.
    pub fn at(&self, x: i32, y: i32, z: i32) -> BlockId {
        BlockId(self.server.block_at(x, y, z))
    }

    /// Places `block` at `(x, y, z)` and shows it to the players.
    pub fn set(&self, x: i32, y: i32, z: i32, block: impl Into<BlockId>) {
        self.server.set_block(x, y, z, block.into().0);
    }

    /// Places many blocks at once, as `(x, y, z, block)`, and answers how many changed. Much faster than calling [`set`](Self::set) for each.
    pub fn set_many<B: Into<BlockId> + Copy>(&self, edits: &[(i32, i32, i32, B)]) -> u32 {
        let edits: Vec<(i32, i32, i32, u32)> = edits
            .iter()
            .map(|&(x, y, z, block)| (x, y, z, block.into().0))
            .collect();
        self.server.set_blocks(&edits)
    }

    /// The default state of the block named `name`, such as `minecraft:stone`, or `None` when the game has no such block.
    pub fn id(&self, name: &str) -> Option<BlockId> {
        let id = self.server.block_runtime_id(name);
        let air = name.strip_prefix("minecraft:").unwrap_or(name) == "air";
        (id != 0 || air).then_some(BlockId(id))
    }

    /// The name of a block, such as `minecraft:stone`.
    pub fn name(&self, block: impl Into<BlockId>) -> String {
        self.server.block_name(block.into().0)
    }

    /// Every state of the block named `name`, such as the eight growth stages of `minecraft:wheat`.
    pub fn variants(&self, name: &str) -> Vec<BlockId> {
        self.server
            .block_variants(name)
            .into_iter()
            .map(BlockId)
            .collect()
    }

    /// The states of a block, as `(name, value)`, such as `("growth", "7")`.
    pub fn states(&self, block: impl Into<BlockId>) -> Vec<(String, String)> {
        self.server.block_states(block.into().0)
    }

    /// The name of every block of the game, sorted, without the `minecraft:` prefix.
    pub fn names(&self) -> Vec<String> {
        self.server.block_names()
    }

    /// The 27 slots of the chest at `(x, y, z)`, or none when there is no chest there.
    pub fn chest(&self, x: i32, y: i32, z: i32) -> Vec<ItemStack> {
        self.server.chest_at(x, y, z)
    }

    /// Replaces what the chest at `(x, y, z)} holds; the slots past `slots` are emptied.
    pub fn set_chest(&self, x: i32, y: i32, z: i32, slots: &[ItemStack]) {
        self.server.set_chest(x, y, z, slots);
    }

    /// Drops `item` at `(x, y, z)` of the world named `world` (empty: the default world), for players to pick up. Answers whether it was dropped.
    pub fn drop_item(&self, world: &str, x: f32, y: f32, z: f32, item: &Item) -> bool {
        self.server.drop_item(world, x, y, z, item)
    }
}

/// Items, from [`Server::items`].
#[derive(Clone, Copy)]
pub struct Items {
    server: Server,
}

impl Items {
    /// The id of the item named `name`, such as `minecraft:diamond`, or `None` when the game has no such item.
    pub fn id(&self, name: &str) -> Option<ItemId> {
        self.server.item_id(name).map(ItemId)
    }

    /// The name of an item, such as `minecraft:diamond`, or an empty string for an unknown id.
    pub fn name(&self, item: impl Into<ItemId>) -> String {
        self.server.item_name(item.into().0)
    }

    /// The durability of a tool, weapon or armor piece, or `None` for an item that does not wear out.
    pub fn max_durability(&self, item: impl Into<ItemId>) -> Option<u16> {
        self.server.max_durability(item.into().0)
    }

    /// How fast `item` mines the blocks of the `tool` family; a bare hand is `1.0`.
    pub fn mining_speed(&self, item: impl Into<ItemId>, tool: ToolType) -> f32 {
        self.server.mining_speed(item.into().0, tool)
    }
}

/// Entities and effects, from [`Server::entities`]. An entity is named by its runtime id.
#[derive(Clone, Copy)]
pub struct Entities {
    server: Server,
}

impl Entities {
    /// Reserves a runtime id for an entity the plugin creates.
    pub fn next_id(&self) -> u64 {
        self.server.next_entity_id()
    }

    /// Makes the client accept a custom entity type, such as one defined in a resource pack.
    pub fn register_type(&self, identifier: &str) -> bool {
        self.server.register_entity_type(identifier)
    }

    /// Adds an entity of type `identifier` at `(x, y, z)`.
    pub fn add(&self, entity_id: u64, identifier: &str, x: f32, y: f32, z: f32) -> ApiResult<()> {
        found(self.server.add_entity(entity_id, identifier, x, y, z))
    }

    /// Removes an entity.
    pub fn remove(&self, entity_id: u64) -> ApiResult<()> {
        found(self.server.remove_entity(entity_id))
    }

    /// Moves an entity to `to` and turns it as it says.
    pub fn teleport(&self, entity_id: u64, to: Location) -> ApiResult<()> {
        found(
            self.server
                .move_entity(entity_id, to.x, to.y, to.z, to.pitch, to.yaw),
        )
    }

    /// Sets the name tag shown above an entity; `\n` starts a new line.
    pub fn set_nametag(&self, entity_id: u64, nametag: &str) -> ApiResult<()> {
        found(self.server.set_entity_nametag(entity_id, nametag))
    }

    /// A snapshot of an entity.
    pub fn get(&self, entity_id: u64) -> Option<Entity> {
        self.server.entity(entity_id)
    }

    /// A snapshot of every entity plugins added.
    pub fn all(&self) -> Vec<Entity> {
        self.server.all_entities()
    }

    /// A snapshot of every entity of type `identifier`.
    pub fn of_type(&self, identifier: &str) -> Vec<Entity> {
        self.server.entities_of(identifier)
    }

    /// Where an entity is, or `None` when there is no such entity.
    pub fn location(&self, entity_id: u64) -> Option<Location> {
        self.server.entity_position(entity_id).map(Location::from)
    }

    /// The name of the world an entity is in.
    pub fn world(&self, entity_id: u64) -> String {
        self.server.entity_world(entity_id)
    }

    /// Moves an entity to the loaded world named `world`, at the same coordinates.
    pub fn set_world(&self, entity_id: u64, world: &str) -> ApiResult<()> {
        found(self.server.set_entity_world(entity_id, world))
    }

    /// Damages an entity by `amount` half-hearts and answers its health left, or `None` when there is no such entity.
    pub fn damage(&self, entity_id: u64, amount: f32) -> Option<f32> {
        let health = self.server.damage_entity(entity_id, amount);
        (health >= 0.0).then_some(health)
    }

    /// Damages an entity as a hit from `(from_x, from_z)` would, knocking it back when `knockback` is set. Answers its health left.
    pub fn damage_from(
        &self,
        entity_id: u64,
        amount: f32,
        from_x: f32,
        from_z: f32,
        knockback: bool,
    ) -> Option<f32> {
        let health = self
            .server
            .damage_entity_from(entity_id, amount, from_x, from_z, knockback);
        (health >= 0.0).then_some(health)
    }

    /// Heals an entity by `amount` half-hearts, up to its maximum, and answers its health.
    pub fn heal(&self, entity_id: u64, amount: f32) -> Option<f32> {
        let health = self.server.heal_entity(entity_id, amount);
        (health >= 0.0).then_some(health)
    }

    /// Sets the health and the maximum health of an entity.
    pub fn set_health(&self, entity_id: u64, health: f32, max_health: f32) -> ApiResult<()> {
        found(self.server.set_entity_health(entity_id, health, max_health))
    }

    /// The health and the maximum health of an entity, or `None` when there is no such entity.
    pub fn health(&self, entity_id: u64) -> Option<(f32, f32)> {
        let (health, max) = self.server.entity_health(entity_id);
        (health >= 0.0).then_some((health, max))
    }

    /// Turns a flag of an entity on or off.
    pub fn set_flag(&self, entity_id: u64, flag: ActorFlag, value: bool) -> ApiResult<()> {
        found(self.server.set_entity_flag(entity_id, flag, value))
    }

    /// Turns a flag on or off by its Bedrock index, for one [`ActorFlag`] does not list.
    pub fn set_flag_raw(&self, entity_id: u64, flag_bit: u32, value: bool) -> ApiResult<()> {
        found(self.server.set_entity_flag_raw(entity_id, flag_bit, value))
    }

    /// Sets the size of an entity, where `1.0` is the normal size.
    pub fn set_scale(&self, entity_id: u64, scale: f32) -> ApiResult<()> {
        found(self.server.set_entity_scale(entity_id, scale))
    }

    /// Sets the hitbox of an entity, in blocks.
    pub fn set_hitbox(&self, entity_id: u64, width: f32, height: f32) -> ApiResult<()> {
        found(self.server.set_entity_hitbox(entity_id, width, height))
    }

    /// Makes an entity ignore damage, or not.
    pub fn set_invulnerable(&self, entity_id: u64, invulnerable: bool) -> ApiResult<()> {
        found(self.server.set_entity_invulnerable(entity_id, invulnerable))
    }

    /// Gives an effect to a player or an entity for `duration_ticks` ticks, with particles. `amplifier` `0` is level I.
    pub fn apply_effect(
        &self,
        entity_id: u64,
        effect: Effect,
        amplifier: u8,
        duration_ticks: i32,
    ) -> ApiResult<()> {
        found(
            self.server
                .apply_effect(entity_id, effect, amplifier, duration_ticks),
        )
    }

    /// Gives an effect by its name, such as `minecraft:speed`. `ambient` makes the particles faint, as a beacon's are.
    pub fn apply_effect_named(
        &self,
        entity_id: u64,
        name: &str,
        amplifier: u8,
        duration_ticks: i32,
        ambient: bool,
        particles: bool,
    ) -> ApiResult<()> {
        found(self.server.apply_effect_named(
            entity_id,
            name,
            amplifier,
            duration_ticks,
            ambient,
            particles,
        ))
    }

    /// Removes an effect. Answers whether the target had it.
    pub fn remove_effect(&self, entity_id: u64, effect: Effect) -> bool {
        self.server.remove_effect(entity_id, effect)
    }

    /// Removes every effect. Answers whether the target had any.
    pub fn clear_effects(&self, entity_id: u64) -> bool {
        self.server.clear_effects(entity_id)
    }

    /// Whether the target has an effect.
    pub fn has_effect(&self, entity_id: u64, effect: Effect) -> bool {
        self.server.has_effect(entity_id, effect)
    }

    /// The amplifier of an effect, `0` for level I, or `None` when the target does not have it.
    pub fn effect_amplifier(&self, entity_id: u64, effect: Effect) -> Option<u8> {
        self.server.effect_amplifier(entity_id, effect)
    }

    /// The ticks left of an effect, or `None` when the target does not have it.
    pub fn effect_duration(&self, entity_id: u64, effect: Effect) -> Option<i32> {
        self.server.effect_duration(entity_id, effect)
    }

    /// The names of the effects the target has.
    pub fn active_effects(&self, entity_id: u64) -> Vec<String> {
        self.server.active_effects(entity_id)
    }
}
