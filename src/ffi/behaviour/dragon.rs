use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct DragonSpawnEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub respawn: bool,
    pub world: u32,
}

#[repr(C)]
pub struct DragonPhaseEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub phase: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct DragonDeathEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub killer: FfiStr,
    pub first: bool,
    pub experience: u32,
    pub egg: bool,
    pub world: u32,
}

crate::event_data! {
    DragonSpawnEventData = 1045, "dragon_spawn";
    DragonPhaseEventData = 1046, "dragon_phase";
    DragonDeathEventData = 1047, "dragon_death";
}
