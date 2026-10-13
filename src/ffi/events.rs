use super::abi::FfiStr;
use super::event::EventHeader;

#[repr(C)]
pub struct PlayerJoinEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub message: FfiStr,
}

#[repr(C)]
pub struct PlayerQuitEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub message: FfiStr,
}

#[repr(C)]
pub struct PlayerLoginEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub xuid: FfiStr,
    pub uuid: FfiStr,
    pub ip: FfiStr,
    pub client_data_json: FfiStr,
    pub login_json: FfiStr,
    pub authenticated: bool,
}

#[repr(C)]
pub struct PlayerChatEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub source_name: FfiStr,
    pub message: FfiStr,
    pub message_type: u8,
    pub xbox_user_id: FfiStr,
    pub platform_chat_id: FfiStr,
    pub formatted: FfiStr,
}

#[repr(C)]
pub struct PlayerMoveEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub from_x: f32,
    pub from_y: f32,
    pub from_z: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub to_z: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub sneaking: bool,
    pub sprinting: bool,
}

#[repr(C)]
pub struct PlayerCommandEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub command: FfiStr,
}

#[repr(C)]
pub struct PlayerInteractEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub action: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub face: u32,
    pub item_id: i16,
    pub block_runtime_id: u32,
    pub hotbar_slot: i32,
    pub click_x: f32,
    pub click_y: f32,
    pub click_z: f32,
}

#[repr(C)]
pub struct BlockBreakEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub runtime_id: u32,
    pub item_id: i16,
    pub held_item: i16,
    pub held_uid: u64,
    pub final_break: bool,
    pub drop: bool,
    pub drop_item: i16,
    pub drop_count: u8,
    pub held_damage: u16,
}

#[repr(C)]
pub struct BlockPlaceEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub runtime_id: u32,
    /// The face, numbered as [`BlockFace`](crate::math::BlockFace) does.
    pub face: u8,
    pub item_id: i16,
}

#[repr(C)]
pub struct PlayerDropItemEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub item_id: i16,
    pub block_runtime_id: u32,
    pub count: u8,
    pub source_slot: u32,
}

#[repr(C)]
pub struct PlayerPickupItemEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub item_id: i16,
    pub block_runtime_id: u32,
    pub count: u8,
    pub entity_id: u64,
}

#[repr(C)]
pub struct PlayerAnimationEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub action: u32,
    pub data: f32,
}

#[repr(C)]
pub struct PlayerItemHeldEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub old_slot: u32,
    pub new_slot: u32,
    pub item_id: i16,
}

#[repr(C)]
pub struct InventoryOpenEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub window_id: u8,
    pub window_type: u8,
}

#[repr(C)]
pub struct InventoryCloseEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub window_id: u8,
    pub window_type: u8,
    pub server: bool,
}

#[repr(C)]
pub struct InventoryTransactionEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub request_id: i32,
    pub action_count: u32,
    pub actions_json: FfiStr,
}

#[repr(C)]
pub struct BlockPickEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub runtime_id: u32,
    pub item_id: i16,
    pub hotbar_slot: u32,
}

#[repr(C)]
pub struct ArmorEquipEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub item_id: i16,
    pub count: u8,
    pub block_runtime_id: u32,
    pub source_slot: u32,
    pub armor_slot: u32,
}

#[repr(C)]
pub struct RandomTickEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub runtime_id: u32,
}

#[repr(C)]
pub struct EntityDamageByEntityEventData {
    pub header: EventHeader,
    pub attacker_entity_id: u64,
    pub target_entity_id: u64,
    pub weapon_item_id: i16,
    pub damage: f32,
    pub attacker: FfiStr,
    pub knockback_force: f32,
    pub knockback_vertical: f32,
    pub attack_cooldown: u64,
}

#[repr(C)]
pub struct ServerTickEventData {
    pub header: EventHeader,
    pub tick: u64,
}

#[repr(C)]
pub struct PlayerDeathEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub killer: FfiStr,
    pub message: FfiStr,
    pub keep_inventory: bool,
}

#[repr(C)]
pub struct PlayerRespawnEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct PlayerGameModeChangeEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub from: i32,
    pub to: i32,
}

#[repr(C)]
pub struct PlayerConsumeEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub item_id: i16,
    pub food_points: u32,
    pub saturation: f32,
}

#[repr(C)]
pub struct PlayerTeleportEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub from_x: f32,
    pub from_y: f32,
    pub from_z: f32,
    pub to_x: f32,
    pub to_y: f32,
    pub to_z: f32,
}

#[repr(C)]
pub struct BlockBreakProgressEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub runtime_id: u32,
    pub stage: u32,
}

#[repr(C)]
pub struct EntityDeathEventData {
    pub header: EventHeader,
    pub entity_id: u64,
    pub identifier: FfiStr,
}

#[repr(C)]
pub struct EntityInteractEventData {
    pub header: EventHeader,
    pub player_name: FfiStr,
    pub entity_id: u64,
    pub identifier: FfiStr,
    pub action: u32,
}

#[repr(C)]
pub struct ServerBoundEventData {
    pub header: EventHeader,
    pub entity_id: u64,
    pub name: FfiStr,
    pub packet_id: u32,
    pub data: FfiStr,
    /// Name of the player the packet comes from or goes to.
    pub player: FfiStr,
}

#[repr(C)]
pub struct ClientBoundEventData {
    pub header: EventHeader,
    pub entity_id: u64,
    pub name: FfiStr,
    pub packet_id: u32,
    pub data: FfiStr,
    /// Name of the player the packet comes from or goes to.
    pub player: FfiStr,
}

#[repr(C)]
pub struct ItemUseEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub item_id: i16,
    pub block_runtime_id: u32,
    pub action: u32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub face: u8,
    pub hotbar_slot: i32,
    pub click_x: f32,
    pub click_y: f32,
    pub click_z: f32,
}

#[repr(C)]
pub struct PlayerToggleEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub kind: u32,
    pub active: bool,
}

#[repr(C)]
pub struct FurnaceSmeltEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub input_item_id: i16,
    pub result_item_id: i16,
}

#[repr(C)]
pub struct ChunkLoadEventData {
    pub header: EventHeader,
    pub cx: i32,
    pub cz: i32,
    pub generated: bool,
}

#[repr(C)]
pub struct ChunkUnloadEventData {
    pub header: EventHeader,
    pub cx: i32,
    pub cz: i32,
}

#[repr(C)]
pub struct WeatherChangeEventData {
    pub header: EventHeader,
    pub kind: u8,
    pub level: f32,
}

#[repr(C)]
pub struct EntitySpawnEventData {
    pub header: EventHeader,
    pub entity_id: u64,
    pub identifier: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub world_id: u32,
}

#[repr(C)]
pub struct EntityDespawnEventData {
    pub header: EventHeader,
    pub entity_id: u64,
    pub identifier: FfiStr,
}

#[repr(C)]
pub struct WorldSaveEventData {
    pub header: EventHeader,
    pub name: FfiStr,
}

#[repr(C)]
pub struct BucketUseEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub item_id: i16,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub filling: bool,
}

#[repr(C)]
pub struct PlayerEmoteEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub entity_id: u64,
    pub emote_id: FfiStr,
    pub duration: u32,
    pub xuid: FfiStr,
    pub platform_id: FfiStr,
    pub flags: u8,
}

#[repr(C)]
pub struct PlayerSkinChangeEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub skin: FfiStr,
    pub new_skin_name: FfiStr,
    pub old_skin_name: FfiStr,
}

#[repr(C)]
pub struct FormResponseEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub form_id: u32,
    pub response: FfiStr,
    pub closed: bool,
}

#[repr(C)]
pub struct ServerSettingsRequestEventData {
    pub header: EventHeader,
    pub name: FfiStr,
}

// The stable numbers of the core events. A number is never reused, even once its event is gone;
// behaviour events are numbered from 1000.
crate::event_data! {
    PlayerLoginEventData = 1, "login";
    PlayerJoinEventData = 2, "join";
    PlayerQuitEventData = 3, "quit";
    PlayerChatEventData = 4, "chat";
    PlayerMoveEventData = 5, "move";
    PlayerCommandEventData = 6, "command";
    PlayerInteractEventData = 7, "interact";
    BlockBreakEventData = 8, "block_break";
    BlockPlaceEventData = 9, "block_place";
    PlayerDropItemEventData = 10, "drop_item";
    PlayerPickupItemEventData = 11, "pickup_item";
    EntityDamageByEntityEventData = 12, "entity_damage";
    ServerTickEventData = 13, "tick";
    PlayerAnimationEventData = 14, "animation";
    PlayerItemHeldEventData = 15, "item_held";
    InventoryOpenEventData = 16, "inventory_open";
    InventoryCloseEventData = 17, "inventory_close";
    InventoryTransactionEventData = 18, "inventory_transaction";
    BlockPickEventData = 19, "block_pick";
    PlayerDeathEventData = 20, "death";
    PlayerRespawnEventData = 21, "respawn";
    PlayerGameModeChangeEventData = 22, "gamemode_change";
    PlayerConsumeEventData = 23, "consume";
    PlayerTeleportEventData = 24, "teleport";
    BlockBreakProgressEventData = 25, "block_break_progress";
    EntityInteractEventData = 26, "entity_interact";
    EntityDeathEventData = 27, "entity_death";
    ServerBoundEventData = 28, "server_bound";
    ClientBoundEventData = 29, "client_bound";
    ItemUseEventData = 30, "item_use";
    PlayerToggleEventData = 31, "toggle";
    FurnaceSmeltEventData = 32, "furnace_smelt";
    ChunkLoadEventData = 33, "chunk_load";
    ChunkUnloadEventData = 34, "chunk_unload";
    WeatherChangeEventData = 35, "weather_change";
    EntitySpawnEventData = 36, "entity_spawn";
    EntityDespawnEventData = 37, "entity_despawn";
    WorldSaveEventData = 38, "world_save";
    BucketUseEventData = 39, "bucket_use";
    PlayerEmoteEventData = 40, "emote";
    PlayerSkinChangeEventData = 41, "skin_change";
    FormResponseEventData = 42, "form_response";
    ServerSettingsRequestEventData = 43, "server_settings_request";
    ArmorEquipEventData = 44, "armor_equip";
    RandomTickEventData = 45, "random_tick";
}

/// The stable number and name of every core event.
pub const CORE_EVENTS: &[(u32, &str)] = &[
    (1, "login"),
    (2, "join"),
    (3, "quit"),
    (4, "chat"),
    (5, "move"),
    (6, "command"),
    (7, "interact"),
    (8, "block_break"),
    (9, "block_place"),
    (10, "drop_item"),
    (11, "pickup_item"),
    (12, "entity_damage"),
    (13, "tick"),
    (14, "animation"),
    (15, "item_held"),
    (16, "inventory_open"),
    (17, "inventory_close"),
    (18, "inventory_transaction"),
    (19, "block_pick"),
    (20, "death"),
    (21, "respawn"),
    (22, "gamemode_change"),
    (23, "consume"),
    (24, "teleport"),
    (25, "block_break_progress"),
    (26, "entity_interact"),
    (27, "entity_death"),
    (28, "server_bound"),
    (29, "client_bound"),
    (30, "item_use"),
    (31, "toggle"),
    (32, "furnace_smelt"),
    (33, "chunk_load"),
    (34, "chunk_unload"),
    (35, "weather_change"),
    (36, "entity_spawn"),
    (37, "entity_despawn"),
    (38, "world_save"),
    (39, "bucket_use"),
    (40, "emote"),
    (41, "skin_change"),
    (42, "form_response"),
    (43, "server_settings_request"),
    (44, "armor_equip"),
    (45, "random_tick"),
];
