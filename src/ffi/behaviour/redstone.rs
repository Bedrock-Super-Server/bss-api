use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct RedstonePowerEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub old_power: i32,
    pub new_power: i32,
    pub world: u32,
}

#[repr(C)]
pub struct PistonMoveEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub extending: bool,
    pub count: u32,
    pub world: u32,
}

#[repr(C)]
pub struct PistonPushEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub entity: u64,
    pub player: FfiStr,
    pub motion_x: f32,
    pub motion_y: f32,
    pub motion_z: f32,
    pub world: u32,
}

#[repr(C)]
pub struct MinecartActivateEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rider: u64,
    pub world: u32,
}

#[repr(C)]
pub struct MinecartTransferEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub item: i16,
    pub into_cart: bool,
    pub world: u32,
}

#[repr(C)]
pub struct MinecartPickupEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub item_entity: u64,
    pub item: i16,
    pub world: u32,
}

#[repr(C)]
pub struct BlockDispenseEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub item: i16,
    pub dropper: bool,
    pub world: u32,
}

#[repr(C)]
pub struct ItemTransferEventData {
    pub header: EventHeader,
    pub from_x: i32,
    pub from_y: i32,
    pub from_z: i32,
    pub to_x: i32,
    pub to_y: i32,
    pub to_z: i32,
    pub item: i16,
    pub world: u32,
}

#[repr(C)]
pub struct HopperPickupEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub entity: u64,
    pub item: i16,
    pub world: u32,
}

#[repr(C)]
pub struct TripwireEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub powered: bool,
    pub player: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct DetectorRailEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub entity: u64,
    pub active: bool,
    pub world: u32,
}

#[repr(C)]
pub struct SculkVibrationEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub source_x: i32,
    pub source_y: i32,
    pub source_z: i32,
    pub frequency: i32,
    pub power: i32,
    pub world: u32,
}

#[repr(C)]
pub struct CrafterCraftEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub recipe: u32,
    pub item: i16,
    pub count: u32,
    pub world: u32,
}

#[repr(C)]
pub struct DispenserEquipEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub player: FfiStr,
    pub item: i16,
    pub slot: u32,
    pub world: u32,
}

#[repr(C)]
pub struct PressurePlateEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub signal: i32,
    pub player: FfiStr,
    pub world: u32,
}

crate::event_data! {
    RedstonePowerEventData = 1026, "redstone_power";
    PistonMoveEventData = 1027, "piston_move";
    PistonPushEventData = 1028, "piston_push";
    MinecartActivateEventData = 1029, "minecart_activate";
    MinecartTransferEventData = 1030, "minecart_transfer";
    MinecartPickupEventData = 1031, "minecart_pickup";
    BlockDispenseEventData = 1032, "block_dispense";
    ItemTransferEventData = 1033, "item_transfer";
    HopperPickupEventData = 1034, "hopper_pickup";
    TripwireEventData = 1035, "tripwire";
    DetectorRailEventData = 1036, "detector_rail";
    SculkVibrationEventData = 1037, "sculk_vibration";
    CrafterCraftEventData = 1038, "crafter_craft";
    DispenserEquipEventData = 1039, "dispenser_equip";
    PressurePlateEventData = 1040, "pressure_plate";
}
