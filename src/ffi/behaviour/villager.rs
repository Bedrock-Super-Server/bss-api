use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct VillagerProfessionEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub profession: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub world: u32,
}

#[repr(C)]
pub struct VillagerTradeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub offer: u32,
    pub times: u32,
    pub item: i16,
    pub experience: i32,
}

#[repr(C)]
pub struct VillagerLevelUpEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub level: u32,
    pub profession: FfiStr,
    pub world: u32,
}

crate::event_data! {
    VillagerProfessionEventData = 1094, "villager_profession";
    VillagerTradeEventData = 1095, "villager_trade";
    VillagerLevelUpEventData = 1096, "villager_level_up";
}
