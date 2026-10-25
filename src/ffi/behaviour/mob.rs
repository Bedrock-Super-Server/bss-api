use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct MobSpawnEventData {
    pub header: EventHeader,
    pub identifier: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub cause: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct MobDespawnEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct MobTargetEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub target: u64,
    pub world: u32,
}

#[repr(C)]
pub struct MobAttackEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub target: u64,
    pub damage: f32,
    pub world: u32,
}

#[repr(C)]
pub struct MobBreedEventData {
    pub header: EventHeader,
    pub parent_a: u64,
    pub parent_b: u64,
    pub identifier: FfiStr,
    pub experience: u32,
    pub world: u32,
}

#[repr(C)]
pub struct MobInteractEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub identifier: FfiStr,
    pub item: i16,
    pub action: FfiStr,
}

#[repr(C)]
pub struct MobTameEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub identifier: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct MobLeashEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub identifier: FfiStr,
    pub leashed: bool,
    pub fence: bool,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub world: u32,
}

crate::event_data! {
    MobSpawnEventData = 1086, "mob_spawn";
    MobDespawnEventData = 1087, "mob_despawn";
    MobTargetEventData = 1088, "mob_target";
    MobAttackEventData = 1089, "mob_attack";
    MobBreedEventData = 1090, "mob_breed";
    MobInteractEventData = 1091, "mob_interact";
    MobTameEventData = 1092, "mob_tame";
    MobLeashEventData = 1093, "mob_leash";
}
