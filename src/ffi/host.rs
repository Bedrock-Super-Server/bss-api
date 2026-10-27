use core::ffi::c_void;

use crate::ffi::abi::*;

/**
The functions the server hands to a plugin.

The server keeps this table in a `static` for as long as the process runs, so a `*const HostApi` received in `on_enable` stays valid for the plugin's whole life.

Every function upholds the same contract, which is what makes calling one sound:

- Scalar arguments may hold any value: an unknown handle, entity id or name is answered with a neutral result, never undefined behaviour.
- An [`FfiStr`] or a `(pointer, length)` pair the plugin passes is only read during the call, so it needs to stay valid until the call returns, nothing longer. A null pointer is read as empty.
- An out-pointer, or an output buffer passed with its capacity, must be valid for writes for that capacity during the call. The host writes at most that many items and returns how many it wrote.
- An [`FfiStr`] the host returns points into a buffer owned by the calling thread. It stays valid until the end of the server callback the plugin is running in or, from a thread the plugin spawned, until that thread's next host call. Copy it before then.
- A callback and its `user_data` must stay callable until the host has invoked it for the last time, which is documented on the registering function.
*/
#[repr(C)]
pub struct HostApi {
    /// Size and version of this table: functions past `header.size` are absent.
    pub header: crate::ffi::version::HostHeader,
    pub player_by_name: Option<unsafe extern "C" fn(name: FfiStr) -> u64>,
    pub player_by_entity_id: Option<unsafe extern "C" fn(entity_id: u64) -> u64>,
    pub player_count: Option<unsafe extern "C" fn() -> u32>,
    pub player_all: Option<unsafe extern "C" fn(out: *mut u64, max: u32) -> u32>,
    pub player_name: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    pub player_xuid: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    pub player_uuid: Option<unsafe extern "C" fn(handle: u64) -> Uuid16>,
    pub player_skin: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    pub player_entity_id: Option<unsafe extern "C" fn(handle: u64) -> u64>,
    pub player_gamemode: Option<unsafe extern "C" fn(handle: u64) -> i32>,
    pub player_health: Option<unsafe extern "C" fn(handle: u64) -> f32>,
    pub player_pos: Option<
        unsafe extern "C" fn(
            handle: u64,
            x: *mut f32,
            y: *mut f32,
            z: *mut f32,
            pitch: *mut f32,
            yaw: *mut f32,
        ),
    >,
    pub player_held: Option<unsafe extern "C" fn(handle: u64) -> HeldItem>,
    pub player_set_pos:
        Option<unsafe extern "C" fn(handle: u64, x: f32, y: f32, z: f32, pitch: f32, yaw: f32)>,
    pub player_teleport:
        Option<unsafe extern "C" fn(handle: u64, x: f32, y: f32, z: f32, pitch: f32, yaw: f32)>,
    pub player_set_health: Option<unsafe extern "C" fn(handle: u64, health: f32)>,
    pub player_hurt: Option<
        unsafe extern "C" fn(
            handle: u64,
            amount: f32,
            from_x: f32,
            from_z: f32,
            kb_force: f32,
            kb_vertical: f32,
            killer: FfiStr,
        ) -> bool,
    >,
    pub player_set_gamemode: Option<unsafe extern "C" fn(handle: u64, mode: i32)>,
    pub player_set_nametag: Option<unsafe extern "C" fn(handle: u64, tag: FfiStr)>,
    pub player_request_give:
        Option<unsafe extern "C" fn(handle: u64, item_id: i16, block_runtime_id: u32, count: u8)>,
    pub player_request_clear_inventory: Option<unsafe extern "C" fn(handle: u64)>,
    pub player_request_set_held: Option<
        unsafe extern "C" fn(handle: u64, item_id: i16, block_runtime_id: u32, count: u8, uid: u64),
    >,
    pub player_send_packet: Option<unsafe extern "C" fn(handle: u64, data: *const u8, len: usize)>,
    pub player_permissions:
        Option<unsafe extern "C" fn(handle: u64, out: *mut FfiStr, max: u32) -> u32>,
    pub player_has_permission:
        Option<unsafe extern "C" fn(handle: u64, permission: FfiStr) -> bool>,
    pub player_add_permission:
        Option<unsafe extern "C" fn(handle: u64, permission: FfiStr) -> bool>,
    pub player_remove_permission:
        Option<unsafe extern "C" fn(handle: u64, permission: FfiStr) -> bool>,
    pub world_block_at: Option<unsafe extern "C" fn(x: i32, y: i32, z: i32) -> u32>,
    pub world_set_block: Option<unsafe extern "C" fn(x: i32, y: i32, z: i32, runtime_id: u32)>,
    pub world_chunk_count: Option<unsafe extern "C" fn() -> u32>,
    pub world_time: Option<unsafe extern "C" fn() -> i64>,
    pub world_set_time: Option<unsafe extern "C" fn(time: i64)>,
    pub world_seed: Option<unsafe extern "C" fn() -> u64>,
    pub world_get_spawn:
        Option<unsafe extern "C" fn(out_x: *mut i32, out_y: *mut i32, out_z: *mut i32) -> bool>,
    pub world_get_chest:
        Option<unsafe extern "C" fn(x: i32, y: i32, z: i32, out: *mut ItemStack, max: u32) -> u32>,
    pub world_set_chest:
        Option<unsafe extern "C" fn(x: i32, y: i32, z: i32, slots: *const ItemStack, count: u32)>,
    pub player_request_inventory: Option<
        unsafe extern "C" fn(handle: u64, callback: InventoryCallback, user_data: *mut c_void),
    >,
    pub inventory_get: Option<unsafe extern "C" fn(inv: InventoryHandle, slot: u32) -> ItemStack>,
    pub inventory_set:
        Option<unsafe extern "C" fn(inv: InventoryHandle, slot: u32, stack: ItemStack)>,
    pub inventory_size: Option<unsafe extern "C" fn(inv: InventoryHandle) -> u32>,
    pub player_open_virtual_chest: Option<
        unsafe extern "C" fn(
            handle: u64,
            title: FfiStr,
            items: *const VirtualSlot,
            count: u32,
            callback: VirtualChestCallback,
            user_data: *mut c_void,
        ) -> bool,
    >,
    pub player_update_virtual_slots:
        Option<unsafe extern "C" fn(handle: u64, slots: *const VirtualSlot, count: u32)>,
    pub form_send_modal: Option<
        unsafe extern "C" fn(
            handle: u64,
            title: FfiStr,
            content: FfiStr,
            button1: FfiStr,
            button2: FfiStr,
            callback: FormCallback,
            user_data: *mut c_void,
        ) -> u32,
    >,
    pub form_send_menu: Option<
        unsafe extern "C" fn(
            handle: u64,
            title: FfiStr,
            content: FfiStr,
            buttons: *const FfiStr,
            button_count: u32,
            callback: FormCallback,
            user_data: *mut c_void,
        ) -> u32,
    >,
    pub form_send_custom: Option<
        unsafe extern "C" fn(
            handle: u64,
            form_json: FfiStr,
            callback: FormCallback,
            user_data: *mut c_void,
        ) -> u32,
    >,
    pub form_send_raw: Option<
        unsafe extern "C" fn(
            handle: u64,
            form_json: FfiStr,
            callback: FormCallback,
            user_data: *mut c_void,
        ) -> u32,
    >,
    pub form_close: Option<unsafe extern "C" fn(handle: u64)>,
    pub broadcast: Option<unsafe extern "C" fn(data: *const u8, len: usize)>,
    pub broadcast_chunk:
        Option<unsafe extern "C" fn(cx: i32, cz: i32, data: *const u8, len: usize)>,
    pub next_entity_id: Option<unsafe extern "C" fn() -> u64>,
    pub block_runtime_id: Option<unsafe extern "C" fn(name: FfiStr) -> u32>,
    pub block_name: Option<unsafe extern "C" fn(runtime_id: u32) -> FfiStr>,
    pub item_id: Option<unsafe extern "C" fn(name: FfiStr) -> i16>,
    pub item_name: Option<unsafe extern "C" fn(item_id: i16) -> FfiStr>,
    pub audit_item: Option<
        unsafe extern "C" fn(
            action: FfiStr,
            player: FfiStr,
            gamemode: i32,
            uid: u64,
            item_id: i16,
            count: u8,
            extra: FfiStr,
        ),
    >,
    pub config_max_players: Option<unsafe extern "C" fn() -> u32>,
    pub config_server_name: Option<unsafe extern "C" fn() -> FfiStr>,
    pub config_tps: Option<unsafe extern "C" fn() -> f64>,
    pub software_name: Option<unsafe extern "C" fn() -> FfiStr>,
    pub software_version: Option<unsafe extern "C" fn() -> FfiStr>,
    pub minecraft_version: Option<unsafe extern "C" fn() -> FfiStr>,
    pub minecraft_protocol: Option<unsafe extern "C" fn() -> FfiStr>,
    pub permission_all: Option<unsafe extern "C" fn(out: *mut FfiStr, max: u32) -> u32>,
    pub command_register: Option<
        unsafe extern "C" fn(
            name: FfiStr,
            description: FfiStr,
            permission: FfiStr,
            overloads_json: FfiStr,
            callback: CommandCallback,
            user_data: *mut c_void,
        ),
    >,
    pub send_raw_packet_all: Option<unsafe extern "C" fn(data: *const u8, len: usize)>,
    pub send_raw_packet: Option<unsafe extern "C" fn(handle: u64, data: *const u8, len: usize)>,
    pub content_register_item: Option<unsafe extern "C" fn(json: FfiStr)>,
    pub content_register_block: Option<unsafe extern "C" fn(json: FfiStr)>,
    pub content_register_group: Option<unsafe extern "C" fn(json: FfiStr)>,
    pub recipe_register: Option<unsafe extern "C" fn(json: FfiStr)>,
    pub item_mining_speed: Option<unsafe extern "C" fn(item_id: i16, tool: u8) -> f32>,
    pub block_names_all: Option<unsafe extern "C" fn(out: *mut FfiStr, max: u32) -> u32>,
    pub block_variants: Option<unsafe extern "C" fn(name: FfiStr, out: *mut u32, max: u32) -> u32>,
    pub block_states: Option<
        unsafe extern "C" fn(
            runtime_id: u32,
            out_keys: *mut FfiStr,
            out_values: *mut FfiStr,
            max: u32,
        ) -> u32,
    >,
    pub world_set_blocks: Option<unsafe extern "C" fn(edits: *const BlockEdit, len: u32) -> u32>,
    pub entity_add: Option<
        unsafe extern "C" fn(entity_id: u64, identifier: FfiStr, x: f32, y: f32, z: f32) -> bool,
    >,
    pub entity_remove: Option<unsafe extern "C" fn(entity_id: u64) -> bool>,
    pub entity_move: Option<
        unsafe extern "C" fn(entity_id: u64, x: f32, y: f32, z: f32, pitch: f32, yaw: f32) -> bool,
    >,
    pub entity_set_nametag: Option<unsafe extern "C" fn(entity_id: u64, nametag: FfiStr) -> bool>,
    pub entity_get: Option<unsafe extern "C" fn(entity_id: u64) -> EntityData>,
    pub entity_all: Option<unsafe extern "C" fn(out: *mut EntityData, max: u32) -> u32>,
    pub entity_register_type: Option<unsafe extern "C" fn(identifier: FfiStr) -> bool>,
    pub world_is_solid: Option<unsafe extern "C" fn(x: i32, y: i32, z: i32) -> bool>,
    pub world_ground_height:
        Option<unsafe extern "C" fn(x: i32, from_y: i32, z: i32, max_fall: i32) -> i32>,
    pub entity_damage: Option<
        unsafe extern "C" fn(
            entity_id: u64,
            amount: f32,
            from_x: f32,
            from_z: f32,
            knockback: bool,
        ) -> f32,
    >,
    pub entity_heal: Option<unsafe extern "C" fn(entity_id: u64, amount: f32) -> f32>,
    pub entity_set_health:
        Option<unsafe extern "C" fn(entity_id: u64, health: f32, max_health: f32) -> bool>,
    pub entity_health: Option<unsafe extern "C" fn(entity_id: u64, out_max: *mut f32) -> f32>,
    pub entity_set_invulnerable:
        Option<unsafe extern "C" fn(entity_id: u64, invulnerable: bool) -> bool>,
    /// Subscribes `callback` to the event numbered `id` (see [`crate::ffi::event`]) at `priority`,
    /// with the `flags` of that module. Returns a token for [`event_unsubscribe`](Self::event_unsubscribe),
    /// or `0` when the server does not know the event. `callback` and `user_data` stay in use until
    /// the plugin unsubscribes or is unloaded.
    pub event_subscribe: Option<
        unsafe extern "C" fn(
            id: u32,
            priority: i32,
            flags: u32,
            callback: crate::ffi::event::EventCallback,
            user_data: *mut c_void,
        ) -> u64,
    >,
    /// Ends the subscription `token` names. Returns `false` when there is none.
    pub event_unsubscribe: Option<unsafe extern "C" fn(token: u64) -> bool>,
    pub player_kick: Option<unsafe extern "C" fn(handle: u64, message: FfiStr)>,
    pub player_ping: Option<unsafe extern "C" fn(handle: u64) -> u32>,
    pub player_ip: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    pub player_set_attribute: Option<unsafe extern "C" fn(handle: u64, attribute: u32, value: f32)>,
    pub player_attribute: Option<unsafe extern "C" fn(handle: u64, attribute: u32) -> f32>,
    pub player_set_armor:
        Option<unsafe extern "C" fn(handle: u64, slot: u8, item_id: i16, block_runtime_id: u32)>,
    pub player_set_flight: Option<
        unsafe extern "C" fn(
            handle: u64,
            may_fly: bool,
            flying: bool,
            fly_speed: f32,
            walk_speed: f32,
        ),
    >,
    pub player_give_item: Option<
        unsafe extern "C" fn(
            handle: u64,
            item_id: i16,
            block_runtime_id: u32,
            count: u8,
            damage: u16,
            name: FfiStr,
            lore_json: FfiStr,
            ench_json: FfiStr,
            unbreakable: bool,
        ),
    >,
    pub inventory_set_item: Option<
        unsafe extern "C" fn(
            inv: InventoryHandle,
            slot: u32,
            item_id: i16,
            block_runtime_id: u32,
            count: u8,
            damage: u16,
            name: FfiStr,
            lore_json: FfiStr,
            ench_json: FfiStr,
            unbreakable: bool,
        ),
    >,
    pub inventory_get_meta: Option<unsafe extern "C" fn(inv: InventoryHandle, slot: u32) -> FfiStr>,
    pub dispatch_command: Option<unsafe extern "C" fn(sender: FfiStr, command: FfiStr) -> bool>,
    pub server_ban:
        Option<unsafe extern "C" fn(target: FfiStr, duration_secs: u64, reason: FfiStr)>,
    pub server_unban: Option<unsafe extern "C" fn(target: FfiStr) -> bool>,
    pub server_whitelist_add: Option<unsafe extern "C" fn(name: FfiStr) -> bool>,
    pub server_whitelist_remove: Option<unsafe extern "C" fn(name: FfiStr) -> bool>,
    pub server_whitelist_set: Option<unsafe extern "C" fn(enabled: bool) -> bool>,
    pub server_op: Option<unsafe extern "C" fn(name: FfiStr) -> bool>,
    pub server_deop: Option<unsafe extern "C" fn(name: FfiStr) -> bool>,
    pub entity_set_flag:
        Option<unsafe extern "C" fn(entity_id: u64, flag_bit: u32, value: bool) -> bool>,
    pub entity_set_scale: Option<unsafe extern "C" fn(entity_id: u64, scale: f32) -> bool>,
    pub entity_set_hitbox:
        Option<unsafe extern "C" fn(entity_id: u64, width: f32, height: f32) -> bool>,
    pub world_biome: Option<unsafe extern "C" fn(x: i32, z: i32) -> FfiStr>,
    pub set_gamerule: Option<unsafe extern "C" fn(name: FfiStr, value: bool)>,
    pub get_gamerule: Option<unsafe extern "C" fn(name: FfiStr, fallback: bool) -> bool>,
    pub world_load_chunk: Option<unsafe extern "C" fn(cx: i32, cz: i32) -> bool>,
    pub service_register: Option<
        unsafe extern "C" fn(name: FfiStr, callback: ServiceCallback, user_data: *mut c_void),
    >,
    pub service_call: Option<unsafe extern "C" fn(name: FfiStr, request: FfiStr) -> FfiStr>,
    pub world_load:
        Option<unsafe extern "C" fn(name: FfiStr, directory: FfiStr, dimension: i32) -> bool>,
    pub world_list: Option<unsafe extern "C" fn(out: *mut FfiStr, max: u32) -> u32>,
    pub world_default: Option<unsafe extern "C" fn() -> FfiStr>,
    pub player_send_to_world: Option<unsafe extern "C" fn(handle: u64, name: FfiStr) -> bool>,
    pub player_world: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    pub player_dimension: Option<unsafe extern "C" fn(handle: u64) -> i32>,
    pub entity_set_world: Option<unsafe extern "C" fn(entity_id: u64, name: FfiStr) -> bool>,
    pub world_block_set:
        Option<unsafe extern "C" fn(world: FfiStr, x: i32, y: i32, z: i32, runtime_id: u32)>,
    pub world_block_get: Option<unsafe extern "C" fn(world: FfiStr, x: i32, y: i32, z: i32) -> u32>,
    pub world_blocks_set:
        Option<unsafe extern "C" fn(world: FfiStr, edits: *const BlockEdit, len: u32) -> u32>,
    pub generator_register: Option<
        unsafe extern "C" fn(name: FfiStr, callback: GeneratorCallback, user_data: *mut c_void),
    >,
    pub chunk_set_block:
        Option<unsafe extern "C" fn(chunk: ChunkHandle, x: i32, y: i32, z: i32, runtime_id: u32)>,
    pub player_set_visibility_group: Option<unsafe extern "C" fn(handle: u64, group: u32)>,
    pub player_visibility_group: Option<unsafe extern "C" fn(handle: u64) -> u32>,
    pub log_line: Option<unsafe extern "C" fn(message: FfiStr)>,
    pub entity_all_of:
        Option<unsafe extern "C" fn(identifier: FfiStr, out: *mut EntityData, max: u32) -> u32>,
    pub entity_pos: Option<
        unsafe extern "C" fn(
            entity_id: u64,
            x: *mut f32,
            y: *mut f32,
            z: *mut f32,
            pitch: *mut f32,
            yaw: *mut f32,
        ) -> bool,
    >,
    pub entity_world: Option<unsafe extern "C" fn(entity_id: u64) -> FfiStr>,
    #[allow(clippy::type_complexity)]
    pub player_set_held_item: Option<
        unsafe extern "C" fn(
            handle: u64,
            item_id: i16,
            block_runtime_id: u32,
            count: u8,
            damage: u16,
            name: FfiStr,
            lore_json: FfiStr,
            ench_json: FfiStr,
            unbreakable: bool,
            uid: u64,
        ),
    >,
    pub item_max_durability: Option<unsafe extern "C" fn(item_id: i16) -> u16>,
    pub software_sha: Option<unsafe extern "C" fn() -> FfiStr>,
    #[allow(clippy::type_complexity)]
    pub plugin_list: Option<
        unsafe extern "C" fn(
            out_names: *mut FfiStr,
            out_versions: *mut FfiStr,
            out_shas: *mut FfiStr,
            max: u32,
        ) -> u32,
    >,
    #[allow(clippy::type_complexity)]
    pub effect_apply: Option<
        unsafe extern "C" fn(
            entity_id: u64,
            name: FfiStr,
            amplifier: i32,
            duration: i32,
            ambient: bool,
            particles: bool,
        ) -> bool,
    >,
    pub effect_remove: Option<unsafe extern "C" fn(entity_id: u64, name: FfiStr) -> bool>,
    pub effect_clear: Option<unsafe extern "C" fn(entity_id: u64) -> bool>,
    pub effect_holds: Option<unsafe extern "C" fn(entity_id: u64, name: FfiStr) -> bool>,
    pub effect_amplifier: Option<unsafe extern "C" fn(entity_id: u64, name: FfiStr) -> i32>,
    pub effect_duration: Option<unsafe extern "C" fn(entity_id: u64, name: FfiStr) -> i32>,
    pub effect_active: Option<unsafe extern "C" fn(entity_id: u64, out: *mut u32, max: u32) -> u32>,
    pub effect_name: Option<unsafe extern "C" fn(id: i32) -> FfiStr>,
    #[allow(clippy::type_complexity)]
    pub world_weather: Option<
        unsafe extern "C" fn(
            world: FfiStr,
            raining: *mut bool,
            thundering: *mut bool,
            rain_ticks: *mut u32,
            thunder_ticks: *mut u32,
        ) -> bool,
    >,
    pub world_set_weather: Option<
        unsafe extern "C" fn(world: FfiStr, raining: bool, thundering: bool, duration: u32) -> bool,
    >,
    pub world_strike_lightning:
        Option<unsafe extern "C" fn(world: FfiStr, x: f32, y: f32, z: f32) -> u64>,
    /// The decoded login of a player, as the login event gives it: every token of the login with
    /// its header, claims and signature, as JSON. Empty for an unknown handle.
    pub player_login: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    /// The client data a player logged in with, as JSON. Empty for an unknown handle.
    pub player_client_data: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    /// Sends the packet `name`, built by the server from `fields` (JSON) for the protocol the
    /// player speaks. Returns empty text when it was sent, else what is wrong with it.
    pub packet_send:
        Option<unsafe extern "C" fn(handle: u64, name: FfiStr, fields: FfiStr) -> FfiStr>,
    /// Sends the packet `name` built from `fields` to every player of the world named `world`, or
    /// to every player when `world` is empty. Returns what `packet_send` does.
    pub packet_broadcast:
        Option<unsafe extern "C" fn(world: FfiStr, name: FfiStr, fields: FfiStr) -> FfiStr>,
    /// Decodes the payload of the packet numbered `packet_id`, as `player` speaks it, into
    /// `{"packet": name, "fields": {...}}`, or `{"error": ...}`.
    pub packet_decode: Option<
        unsafe extern "C" fn(player: u64, packet_id: u32, payload: *const u8, len: usize) -> FfiStr,
    >,
    /// The name and number of every packet, as JSON: `[{"id": 9, "name": "Text"}, ...]`.
    pub packet_list: Option<unsafe extern "C" fn() -> FfiStr>,
    /// What each loaded plugin took, as JSON: `[{"name", "calls", "busy_ms", "max_us",
    /// "last_second_percent", "average_percent"}, ...]`, the busiest first.
    pub plugin_usage: Option<unsafe extern "C" fn() -> FfiStr>,
    /// The experience level of a player, or -1 for an unknown handle.
    pub player_experience_level: Option<unsafe extern "C" fn(handle: u64) -> i32>,
    /// Gives a player `levels` experience levels, or takes them when negative, never below 0.
    pub player_add_experience_levels: Option<unsafe extern "C" fn(handle: u64, levels: i32)>,
    /// Drops a stack at `x`, `y`, `z` of the world named `world` (empty: the default world), as
    /// the server drops items. Returns whether it was dropped.
    pub world_drop_item: Option<
        unsafe extern "C" fn(
            world: FfiStr,
            x: f32,
            y: f32,
            z: f32,
            item_id: i16,
            block_runtime_id: u32,
            count: u8,
            damage: u16,
            name: FfiStr,
            lore_json: FfiStr,
            ench_json: FfiStr,
            unbreakable: bool,
        ) -> bool,
    >,
    /// The slots of the player's inventory, as the server last saw them, as a JSON list of
    /// `null` or `{ item_id, block_runtime_id, count, damage, uid, name, lore, enchants,
    /// unbreakable }`.
    pub player_inventory_snapshot: Option<unsafe extern "C" fn(handle: u64) -> FfiStr>,
    /// Adds a recipe of a station, as `{ station, width, height, inputs, output, count }`, whose
    /// inputs may be item tags written `#tag`.
    pub recipe_register_at: Option<unsafe extern "C" fn(json: FfiStr)>,
    /// Sends the HTTP request `request_json`, `{ uri, method, headers: [{ key, value }], body,
    /// timeout }` with the timeout in seconds, from a thread of the server, and calls `callback`
    /// once with `{ status, headers: [{ key, value }], body }` or `{ error }`. Returns whether the
    /// request was sent; `callback` is not called when it was not.
    pub http_request: Option<
        unsafe extern "C" fn(
            request_json: FfiStr,
            callback: HttpCallback,
            user_data: *mut c_void,
        ) -> bool,
    >,
    /// Shows a data-driven screen to a player: `screen_json` describes it (`title` and `elements`), and
    /// `callback` is called with each event of the screen as JSON, then a last time when it closes. Pass
    /// the same `callback` and `user_data` for every screen. Returns the number of the screen, which is
    /// never `0`, or `0` when the screen was refused. A screen the player already had open is closed.
    pub ddui_send: Option<
        unsafe extern "C" fn(
            handle: u64,
            screen_json: FfiStr,
            callback: DduiCallback,
            user_data: *mut c_void,
        ) -> u32,
    >,
    /// Changes one `field` of the element `id` of the open screen `form_id` to `value_json`. Returns
    /// whether the change was made.
    pub ddui_update: Option<
        unsafe extern "C" fn(
            handle: u64,
            form_id: u32,
            id: FfiStr,
            field: FfiStr,
            value_json: FfiStr,
        ) -> bool,
    >,
    /// Replaces the whole content of the open screen `form_id` with `screen_json`. Returns whether it
    /// was replaced.
    pub ddui_replace:
        Option<unsafe extern "C" fn(handle: u64, form_id: u32, screen_json: FfiStr) -> bool>,
    /// Closes the screen `form_id`, or every screen when it is `0`. Returns whether one was closed.
    pub ddui_close: Option<unsafe extern "C" fn(handle: u64, form_id: u32) -> bool>,
    /// Returns the value of the state `key` of the block `runtime_id` as text, or nothing when the block has no such state.
    pub block_state: Option<unsafe extern "C" fn(runtime_id: u32, key: FfiStr) -> FfiStr>,
    /// Returns the runtime id of the block `runtime_id` with its state `key` set to `value`, or `u32::MAX` when no block of
    /// that name has the state at that value.
    pub block_with_state:
        Option<unsafe extern "C" fn(runtime_id: u32, key: FfiStr, value: FfiStr) -> u32>,
    /// Returns the view distance the player has now, in chunks.
    pub player_view_distance: Option<unsafe extern "C" fn(handle: u64) -> i32>,
    /// Returns the view distance the player's client asked for last, in chunks: the most it can be given.
    pub player_requested_view_distance: Option<unsafe extern "C" fn(handle: u64) -> i32>,
    /// Lets the player see at most `chunks` chunks, more or less than the server's `view_distance`; zero or less gives the server's own limit back. The player gets the view distance they asked for within it: it grows or shrinks at once, and is granted the same way when their client asks again. Returns whether the player was found.
    pub player_set_view_distance: Option<unsafe extern "C" fn(handle: u64, chunks: i32) -> bool>,
    /// Returns the `view_distance` of the server, in chunks.
    pub server_view_distance: Option<unsafe extern "C" fn() -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EntityData {
    pub entity_id: u64,
    pub identifier: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub nametag: FfiStr,
    pub scale: f32,
    pub no_gravity: bool,
    pub immobile: bool,
    pub invisible: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct BlockEdit {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub runtime_id: u32,
}

#[repr(C)]
pub struct VirtualSlot {
    pub present: bool,
    pub item_id: i16,
    pub block_runtime_id: u32,
    pub count: u8,
    pub name: FfiStr,
    pub lore_json: FfiStr,
}
