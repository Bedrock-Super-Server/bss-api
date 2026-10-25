use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct EntityPlaceEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub identifier: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct EntityTouchEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub identifier: FfiStr,
    pub attack: bool,
}

#[repr(C)]
pub struct EntityHurtEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub damage: f32,
    pub attacker: u64,
}

#[repr(C)]
pub struct EntityKilledEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub killer: u64,
    pub experience: u32,
    pub drops: bool,
}

#[repr(C)]
pub struct ArmorStandChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub slot: u32,
    pub item: i16,
}

#[repr(C)]
pub struct EntityCombustEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub ticks: u32,
    pub cause: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct ItemSpawnEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub item: i16,
    pub count: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub world: u32,
}

#[repr(C)]
pub struct ItemDespawnEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub item: i16,
    pub count: u32,
}

crate::event_data! {
    EntityPlaceEventData = 1078, "entity_place";
    EntityTouchEventData = 1079, "entity_touch";
    EntityHurtEventData = 1080, "entity_hurt";
    EntityKilledEventData = 1081, "entity_killed";
    ArmorStandChangeEventData = 1082, "armor_stand_change";
    EntityCombustEventData = 1083, "entity_combust";
    ItemSpawnEventData = 1084, "item_spawn";
    ItemDespawnEventData = 1085, "item_despawn";
}
