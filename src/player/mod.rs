//! Players and what a plugin can do with them: messages, inventories, forms, permissions, teleports.

mod attribute;
pub mod client_data;
pub mod ddui;
pub mod form;
pub(crate) mod inventory;
mod item;
pub mod jwt;
pub mod uuid;

pub use attribute::Attribute;
pub use inventory::{Inventory, VirtualItem};
pub use item::{Enchant, Item, ItemMeta};

use crate::content::ArmorSlot;
use crate::ffi::abi::{FfiStr, FormCallback, HeldItem, InventoryCallback, VirtualChestCallback};
use crate::ffi::call::HostFn;
use crate::ffi::host::{HostApi, VirtualSlot};
use crate::player::client_data::ClientData;

use crate::registry::callbacks as ui;
use crate::server::ffi_string;

fn virtual_lore_json(item: &Option<VirtualItem>) -> String {
    item.as_ref()
        .map(|item| serde_json::to_string(&item.lore).unwrap_or_default())
        .unwrap_or_default()
}

/**
A player connected to the server.

A `Player` is a cheap handle: copy it freely. It stays valid after the player leaves; every method then does nothing or returns a neutral value, such as an empty string or `0`.

Get one from [`Server::player`](crate::server::Server::player), [`Server::players`](crate::server::Server::players) or [`CommandContext::player`](crate::command::CommandContext::player).

# Example

```
use bss_api::prelude::*;

fn reward(player: &Player) {
    player.message("§6You found a treasure!");
    if let Some(diamond) = player.server().item_id("minecraft:diamond") {
        player.give(diamond, 0, 3);
    }
    player.set_health(20.0);
}
```
*/
#[derive(Clone, Copy)]
pub struct Player {
    handle: u64,
    host: &'static HostApi,
}

impl Player {
    pub(crate) fn new(handle: u64, host: &'static HostApi) -> Self {
        Self { handle, host }
    }

    /// Returns the runtime id of the player, which identifies them in entity events.
    pub fn entity_id(&self) -> u64 {
        self.handle
    }

    /// Returns the server.
    pub fn server(&self) -> crate::server::Server {
        crate::server::Server::from_host(self.host)
    }

    /// Returns the name of the player.
    pub fn name(&self) -> String {
        host_call!(self.host, player_name(self.handle))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the Xbox user id of the player, or an empty string when the server runs without online authentication.
    pub fn xuid(&self) -> String {
        host_call!(self.host, player_xuid(self.handle))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /**
    Returns the whole login of the player, decoded: the protocol, the authentication info, and
    the identity and client data tokens, each with its raw text, header, claims and signature.

    `None` once the player has left, or on a server too old to provide it.
    */
    pub fn login(&self) -> Option<crate::player::jwt::LoginJwt> {
        let json = host_call!(self.host, player_login(self.handle)).map(ffi_string)?;
        (!json.is_empty()).then(|| crate::player::jwt::LoginJwt::from_json(&json))
    }

    /**
    Returns the client data the player logged in with: device, platform, language
    (`language_code`), game version, input mode, skin and the rest of what the game sends.

    `None` once the player has left, or on a server too old to provide it.
    */
    pub fn client_data(&self) -> Option<ClientData> {
        let json = host_call!(self.host, player_client_data(self.handle)).map(ffi_string)?;
        (!json.is_empty()).then(|| ClientData::from_json(&json))
    }

    /// Returns the UUID of the player.
    pub fn uuid(&self) -> [u8; 16] {
        host_call!(self.host, player_uuid(self.handle))
            .map(|value| value.0)
            .unwrap_or([0; 16])
    }

    /// Returns the skin of the player, in the Bedrock serialized skin format.
    pub fn skin_bytes(&self) -> Vec<u8> {
        host_call!(self.host, player_skin(self.handle))
            .map(crate::server::ffi_bytes)
            .unwrap_or_default()
    }

    /// Returns the game mode: `0` survival, `1` creative, `2` adventure, `6` spectator.
    pub fn gamemode(&self) -> i32 {
        host_call!(self.host, player_gamemode(self.handle)).unwrap_or(0)
    }

    /// Returns the health of the player, in half-hearts: `20.0` is full health.
    pub fn health(&self) -> f32 {
        host_call!(self.host, player_health(self.handle)).unwrap_or(0.0)
    }

    /// Returns the item in the player's main hand.
    pub fn held(&self) -> HeldItem {
        host_call!(self.host, player_held(self.handle)).unwrap_or_default()
    }

    /// Returns the position of the player's feet and the direction they look at, as `(x, y, z, pitch, yaw)`, angles in degrees.
    pub fn position(&self) -> (f32, f32, f32, f32, f32) {
        let (mut x, mut y, mut z, mut pitch, mut yaw) = (0.0, 0.0, 0.0, 0.0, 0.0);
        host_call!(
            self.host,
            player_pos(
                self.handle,
                &raw mut x,
                &raw mut y,
                &raw mut z,
                &raw mut pitch,
                &raw mut yaw
            )
        );
        (x, y, z, pitch, yaw)
    }

    /// Updates the position the server keeps for the player, without moving them on their screen. Use [`teleport`](Self::teleport) to move a player.
    pub fn set_position(&self, x: f32, y: f32, z: f32, pitch: f32, yaw: f32) {
        host_call!(self.host, player_set_pos(self.handle, x, y, z, pitch, yaw));
    }

    /**
    Moves the player's feet to `(x, y, z)`, facing `pitch` and `yaw`, in degrees.

    ```
    # use bss_api::prelude::*;
    fn to_spawn(player: &Player) {
        if let Some((x, y, z)) = player.server().world_spawn() {
            player.teleport(x as f32 + 0.5, y as f32, z as f32 + 0.5, 0.0, 0.0);
        }
    }
    ```
    */
    pub fn teleport(&self, x: f32, y: f32, z: f32, pitch: f32, yaw: f32) {
        host_call!(self.host, player_teleport(self.handle, x, y, z, pitch, yaw));
    }

    /// Changes the game mode, numbered as in [`gamemode`](Self::gamemode).
    pub fn set_gamemode(&self, mode: i32) {
        host_call!(self.host, player_set_gamemode(self.handle, mode));
    }

    /// Disconnects the player, showing `message` on their screen.
    pub fn kick(&self, message: &str) {
        host_call!(self.host, player_kick(self.handle, FfiStr::new(message)));
    }

    /// Returns the round-trip time to the player's client, in milliseconds.
    pub fn ping(&self) -> u32 {
        host_call!(self.host, player_ping(self.handle)).unwrap_or(0)
    }

    /// Returns the view distance the player has now, in chunks.
    pub fn view_distance(&self) -> i32 {
        host_call!(self.host, player_view_distance(self.handle)).unwrap_or(0)
    }

    /// Returns the view distance the player's client asked for last, in chunks. It is the most they can be given, whatever the limit.
    pub fn requested_view_distance(&self) -> i32 {
        host_call!(self.host, player_requested_view_distance(self.handle)).unwrap_or(0)
    }

    /**
    Lets the player see at most `chunks` chunks, more or less than the `view_distance` of the server.

    They get the view distance they asked for within that limit, never more than their client asked: it changes at once, with the chunks sent or dropped, and the same limit answers every later request of their client. Zero or less gives the server's own limit back. The limit is not kept when the player leaves.
    */
    pub fn set_view_distance(&self, chunks: i32) -> bool {
        host_call!(self.host, player_set_view_distance(self.handle, chunks)).unwrap_or(false)
    }

    /// Returns the IP address and port the player connects from.
    pub fn ip(&self) -> String {
        host_call!(self.host, player_ip(self.handle))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the name of the world the player is in.
    pub fn world(&self) -> String {
        host_call!(self.host, player_world(self.handle))
            .map(ffi_string)
            .unwrap_or_default()
    }

    /// Returns the dimension the player is in: `0` overworld, `1` Nether, `2` End.
    pub fn dimension(&self) -> i32 {
        host_call!(self.host, player_dimension(self.handle)).unwrap_or(0)
    }

    /// Moves the player to the loaded world named `name`. Returns `false` when no such world is loaded.
    pub fn send_to_world(&self, name: &str) -> bool {
        host_call!(
            self.host,
            player_send_to_world(self.handle, FfiStr::new(name))
        )
        .unwrap_or(false)
    }

    /// Sets an attribute, clamped to its valid range.
    pub fn set_attribute(&self, attribute: Attribute, value: f32) {
        host_call!(
            self.host,
            player_set_attribute(self.handle, attribute.index(), value)
        );
    }

    /// Returns the current value of an attribute.
    pub fn attribute(&self, attribute: Attribute) -> f32 {
        host_call!(self.host, player_attribute(self.handle, attribute.index())).unwrap_or(0.0)
    }

    /// Puts an item in an armor slot; `item_id` `0` empties it.
    pub fn set_armor(&self, slot: ArmorSlot, item_id: i16, block_runtime_id: u32) {
        let index = match slot {
            ArmorSlot::Head => 0u8,
            ArmorSlot::Chest => 1,
            ArmorSlot::Legs => 2,
            ArmorSlot::Feet => 3,
        };
        host_call!(
            self.host,
            player_set_armor(self.handle, index, item_id, block_runtime_id)
        );
    }

    /// Sets whether the player may fly and is flying, and their flying and walking speeds; `0.0` keeps the default speed.
    pub fn set_abilities(&self, may_fly: bool, flying: bool, fly_speed: f32, walk_speed: f32) {
        host_call!(
            self.host,
            player_set_flight(self.handle, may_fly, flying, fly_speed, walk_speed)
        );
    }

    /// Sets whether the player may fly and is flying, at the default speeds.
    pub fn set_flight(&self, may_fly: bool, flying: bool) {
        self.set_abilities(may_fly, flying, 0.0, 0.0);
    }

    /// Sets the health of the player, in half-hearts.
    pub fn set_health(&self, health: f32) {
        host_call!(self.host, player_set_health(self.handle, health));
    }

    /**
    Damages the player as a melee hit would: armor reduces `amount`, the player is knocked back away from `(from_x, from_z)`, and dies with `killer` named in the death message when their health runs out.

    Returns `false` without hurting a player in creative mode.
    */
    pub fn hurt(
        &self,
        amount: f32,
        from_x: f32,
        from_z: f32,
        kb_force: f32,
        kb_vertical: f32,
        killer: &str,
    ) -> bool {
        host_call!(
            self.host,
            player_hurt(
                self.handle,
                amount,
                from_x,
                from_z,
                kb_force,
                kb_vertical,
                FfiStr::new(killer)
            )
        )
        .unwrap_or(false)
    }

    /// Sets the name shown above the player's head.
    pub fn set_nametag(&self, tag: &str) {
        host_call!(self.host, player_set_nametag(self.handle, FfiStr::new(tag)));
    }

    /// Sends `message` to the player's chat. `§` color codes work.
    pub fn message(&self, message: &str) {
        let _ = self.send_packet(
            "Text",
            &serde_json::json!({
                "localize": false,
                "body": { "MessageOnly": { "message_type": 0, "message": message } },
                "sender_s_xuid": "",
                "platform_id": "",
                "filtered_message": null,
            }),
        );
    }

    /**
    Gives `count` items to the player. `block_runtime_id` is `0` for an item that is not a block.

    The player's session applies the change shortly after the call.
    */
    pub fn give(&self, item_id: i16, block_runtime_id: u32, count: u8) {
        host_call!(
            self.host,
            player_request_give(self.handle, item_id, block_runtime_id, count)
        );
    }

    /// Replaces the item in the player's main hand with `item`, which may carry a name, lore and enchantments. `uid` is the unique id of the stack.
    pub fn set_held_item(&self, item: &Item, uid: u64) {
        if let Some(f) = self.host.player_set_held_item {
            f.call((
                self.handle,
                item.item_id,
                item.block_runtime_id,
                item.count,
                item.damage,
                FfiStr::new(&item.name),
                FfiStr::new(&item.lore_json()),
                FfiStr::new(&item.ench_json()),
                item.unbreakable,
                uid,
            ));
        }
    }

    /// The 36 slots of the player's inventory, hotbar first, with their name, lore and
    /// enchantments, as the server last saw them: at most a tick old, and read at once from any
    /// thread, unlike [`request_inventory`](Self::request_inventory). An empty slot has `count`
    /// `0`. Empty when the player is gone or the server is older than ABI 1.5.
    pub fn inventory_snapshot(&self) -> Vec<(crate::ItemStack, ItemMeta)> {
        #[derive(serde::Deserialize)]
        struct Slot {
            item_id: i16,
            block_runtime_id: u32,
            count: u8,
            damage: u16,
            uid: u64,
            #[serde(flatten)]
            meta: ItemMeta,
        }
        let Some(json) =
            host_call!(self.host, player_inventory_snapshot(self.handle)).map(ffi_string)
        else {
            return Vec::new();
        };
        serde_json::from_str::<Vec<Option<Slot>>>(&json)
            .unwrap_or_default()
            .into_iter()
            .map(|slot| match slot {
                Some(s) => (
                    crate::ItemStack {
                        item_id: s.item_id,
                        block_runtime_id: s.block_runtime_id,
                        count: s.count,
                        damage: s.damage,
                        uid: s.uid,
                    },
                    s.meta,
                ),
                None => (crate::ItemStack::default(), ItemMeta::default()),
            })
            .collect()
    }

    /// The player's experience level, or `None` when the server is too old to tell or the player
    /// is gone.
    pub fn experience_level(&self) -> Option<i32> {
        host_call!(self.host, player_experience_level(self.handle)).filter(|level| *level >= 0)
    }

    /// Gives the player `levels` experience levels, or takes them when negative; the level never
    /// goes below 0. The change applies on the player's next tick.
    pub fn add_experience_levels(&self, levels: i32) {
        host_call!(self.host, player_add_experience_levels(self.handle, levels));
    }

    /**
    Gives `item`, which may carry a name, lore and enchantments.

    ```
    # use bss_api::prelude::*;
    # use bss_api::player::{Enchant, Item};
    fn starter_sword(player: &Player) {
        let Some(id) = player.server().item_id("minecraft:iron_sword") else { return };
        player.give_item(&Item::new(id).name("§bStarter Sword").enchant(Enchant::Sharpness, 2));
    }
    ```
    */
    pub fn give_item(&self, item: &Item) {
        if let Some(f) = self.host.player_give_item {
            f.call((
                self.handle,
                item.item_id,
                item.block_runtime_id,
                item.count,
                item.damage,
                FfiStr::new(&item.name),
                FfiStr::new(&item.lore_json()),
                FfiStr::new(&item.ench_json()),
                item.unbreakable,
            ));
        }
    }

    /// Empties the player's inventory.
    pub fn clear_inventory(&self) {
        host_call!(self.host, player_request_clear_inventory(self.handle));
    }

    /// Replaces the item in the player's main hand with a plain item.
    pub fn set_held(&self, item_id: i16, block_runtime_id: u32, count: u8, uid: u64) {
        host_call!(
            self.host,
            player_request_set_held(self.handle, item_id, block_runtime_id, count, uid)
        );
    }

    /**
    Sends the packet `name` to the player, with `fields` as JSON. The server builds the packet for
    the protocol version the player speaks and checks it the way it checks what players send, so
    the plugin needs neither `bss-protocol` nor a rebuild when the protocol changes.

    The fields are those of the packet as the server decodes it: see
    [`PacketEvent::fields`](crate::event::PacketEvent::fields) and [`Server::packets`](crate::server::Server::packets).
    Returns what is wrong with the packet when it was not sent.

    ```
    use bss_api::prelude::*;

    fn title(player: &Player) -> Result<(), String> {
        player.send_packet(
            "SetTitle",
            &json!({
                "title_type": 2,
                "title_text": "§6Welcome",
                "fade_in_time": 10,
                "stay_time": 40,
                "fade_out_time": 10,
                "xuid": "",
                "platform_online_id": "",
                "filtered_title_message": ""
            }),
        )
    }
    ```
    */
    pub fn send_packet(&self, name: &str, fields: &serde_json::Value) -> Result<(), String> {
        let fields = fields.to_string();
        match host_call!(
            self.host,
            packet_send(self.handle, FfiStr::new(name), FfiStr::new(&fields))
        ) {
            Some(answer) => {
                let error = ffi_string(answer);
                if error.is_empty() { Ok(()) } else { Err(error) }
            }
            None => Err("the server does not send packets built from JSON".to_string()),
        }
    }

    /// Sends a frame the plugin encoded itself: the packet id as an unsigned varint, then the
    /// payload, for the protocol version the player speaks. Prefer [`send_packet`](Player::send_packet).
    pub fn send_raw_packet(&self, frame: &[u8]) {
        host_call!(
            self.host,
            player_send_packet(self.handle, frame.as_ptr(), frame.len())
        );
    }

    /// Puts the player in a visibility group: players see each other only when they share a group, or when one of them is in group `0`, the default. Use it to separate players of different minigames on one world.
    pub fn set_visibility_group(&self, group: u32) {
        host_call!(self.host, player_set_visibility_group(self.handle, group));
    }

    /// Returns the visibility group of the player.
    pub fn visibility_group(&self) -> u32 {
        host_call!(self.host, player_visibility_group(self.handle)).unwrap_or(0)
    }

    /// Returns the permissions the player holds.
    pub fn permissions(&self) -> Vec<String> {
        let Some(f) = self.host.player_permissions else {
            return Vec::new();
        };
        let mut permissions = Vec::new();
        crate::server::read_all(|buffer: &mut [FfiStr]| {
            let count = f.call((self.handle, buffer.as_mut_ptr(), buffer.len() as u32)) as usize;
            permissions = buffer
                .iter()
                .take(count)
                .map(|permission| ffi_string(*permission))
                .collect();
            count
        });
        permissions
    }

    /// Returns whether the player holds `permission`.
    pub fn has_permission(&self, permission: &str) -> bool {
        host_call!(
            self.host,
            player_has_permission(self.handle, FfiStr::new(permission))
        )
        .unwrap_or(false)
    }

    /// Grants `permission`. Returns `false` if the player already had it.
    pub fn add_permission(&self, permission: &str) -> bool {
        host_call!(
            self.host,
            player_add_permission(self.handle, FfiStr::new(permission))
        )
        .unwrap_or(false)
    }

    /// Revokes `permission`. Returns `false` if the player did not have it.
    pub fn remove_permission(&self, permission: &str) -> bool {
        host_call!(
            self.host,
            player_remove_permission(self.handle, FfiStr::new(permission))
        )
        .unwrap_or(false)
    }

    /**
    Shows a form, and calls `handler` once with the answer, or with [`FormResponse::Closed`](crate::player::form::FormResponse::Closed) when the player closes it.

    See [`form`] for an example.
    */
    pub fn send_form(
        &self,
        form: crate::player::form::Form,
        handler: impl FnOnce(crate::player::form::FormResponse) + Send + 'static,
    ) {
        let Some(f) = self.host.form_send_raw else {
            return;
        };
        let json = form.to_json();
        let raw_handler: Box<dyn FnOnce(&str) + Send> =
            Box::new(move |response: &str| handler(form.parse(response)));
        let user_data = ui::box_form(raw_handler);
        f.call((
            self.handle,
            FfiStr::new(&json),
            FormCallback::Some(ui::answer_form),
            user_data,
        ));
    }

    /**
    Shows a form written as the JSON of the game's `ModalFormRequest` packet, and calls `handler`
    once with the player's raw answer, the JSON of `ModalFormResponse`, or `None` when they close
    it. A player in another screen, such as the chat, answers `"busy"` on servers of ABI 1.5 or
    newer. Use it for what [`Form`](crate::player::form::Form) does not describe, such as the headers,
    labels and dividers of newer forms.
    */
    pub fn send_form_json(&self, json: &str, handler: impl FnOnce(Option<&str>) + Send + 'static) {
        let Some(f) = self.host.form_send_raw else {
            return;
        };
        let raw_handler: Box<dyn FnOnce(&str) + Send> = Box::new(move |response: &str| {
            let answer = response.trim();
            handler((!answer.is_empty() && answer != "null").then_some(answer));
        });
        let user_data = ui::box_form(raw_handler);
        f.call((
            self.handle,
            FfiStr::new(json),
            FormCallback::Some(ui::answer_form),
            user_data,
        ));
    }

    /// Closes the form the player is looking at, if any.
    pub fn close_form(&self) {
        host_call!(self.host, form_close(self.handle));
    }

    /**
    Calls `handler` with the player's inventory, on the thread that owns it. The inventory can be read and changed only inside `handler`.

    ```
    # use bss_api::prelude::*;
    fn count_diamonds(player: &Player) {
        let Some(diamond) = player.server().item_id("minecraft:diamond") else { return };
        let who = *player;
        player.request_inventory(move |inventory| {
            let total: u32 = (0..inventory.size()).map(|slot| inventory.get(slot)).filter(|stack| stack.item_id == diamond).map(|stack| u32::from(stack.count)).sum();
            who.message(&format!("You have {total} diamonds."));
        });
    }
    ```
    */
    pub fn request_inventory(&self, handler: impl FnOnce(&Inventory) + Send + 'static) {
        if let Some(f) = self.host.player_request_inventory {
            let user_data = ui::box_inventory(Box::new(handler));
            f.call((
                self.handle,
                InventoryCallback::Some(ui::read_inventory),
                user_data,
            ));
        }
    }

    /**
    Opens a chest window that exists only for this player, filled with `items`, one per slot. `handler` is called with the slot and the player each time they click a slot; the items cannot be taken. Use it to build menus.

    Returns `false` when the server cannot open it.
    */
    pub fn open_virtual_chest(
        &self,
        title: &str,
        items: &[Option<VirtualItem>],
        handler: impl Fn(usize, Player) + Send + Sync + 'static,
    ) -> bool {
        let Some(f) = self.host.player_open_virtual_chest else {
            return false;
        };
        let lore: Vec<String> = items.iter().map(virtual_lore_json).collect();
        let slots: Vec<VirtualSlot> = items
            .iter()
            .zip(&lore)
            .map(|(item, lore)| VirtualItem::slot(item.as_ref(), lore))
            .collect();
        ui::register_chest(self.handle, std::sync::Arc::new(handler));
        f.call((
            self.handle,
            FfiStr::new(title),
            slots.as_ptr(),
            slots.len() as u32,
            VirtualChestCallback::Some(ui::click_chest),
            std::ptr::null_mut(),
        ))
    }

    /// Replaces the items of the virtual chest the player has open.
    pub fn update_virtual_slots(&self, items: &[Option<VirtualItem>]) {
        if let Some(f) = self.host.player_update_virtual_slots {
            let lore: Vec<String> = items.iter().map(virtual_lore_json).collect();
            let slots: Vec<VirtualSlot> = items
                .iter()
                .zip(&lore)
                .map(|(item, lore)| VirtualItem::slot(item.as_ref(), lore))
                .collect();
            f.call((self.handle, slots.as_ptr(), slots.len() as u32));
        }
    }
}
