use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct CraftItemEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub station: FfiStr,
    pub recipe: u32,
    pub item: i16,
    pub count: u32,
}

#[repr(C)]
pub struct EnchantItemEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub option: i32,
    pub levels: i32,
}

#[repr(C)]
pub struct AnvilUseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub name: FfiStr,
    pub levels: i32,
}

#[repr(C)]
pub struct GrindstoneUseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub experience: i32,
}

#[repr(C)]
pub struct SmithingUseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub result: i16,
}

#[repr(C)]
pub struct BeaconChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub primary: i32,
    pub secondary: i32,
}

crate::event_data! {
    CraftItemEventData = 1068, "craft_item";
    EnchantItemEventData = 1069, "enchant_item";
    AnvilUseEventData = 1070, "anvil_use";
    GrindstoneUseEventData = 1071, "grindstone_use";
    SmithingUseEventData = 1072, "smithing_use";
    BeaconChangeEventData = 1073, "beacon_change";
}
