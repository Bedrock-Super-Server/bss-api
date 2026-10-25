use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct SignChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub front: bool,
    pub text: FfiStr,
}

#[repr(C)]
pub struct BlockUseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub item: i16,
    pub action: FfiStr,
}

#[repr(C)]
pub struct BlockGrowEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub new_block: u32,
    pub bone_meal: bool,
    pub world: u32,
}

#[repr(C)]
pub struct TreeGrowEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub sapling: u32,
    pub world: u32,
}

#[repr(C)]
pub struct BlockFallEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub world: u32,
}

#[repr(C)]
pub struct BlockLandEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub world: u32,
}

#[repr(C)]
pub struct BlockFormEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub new_block: u32,
    pub world: u32,
}

#[repr(C)]
pub struct BlockIgniteEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub cause: FfiStr,
}

#[repr(C)]
pub struct TntPrimeEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub fuse: u32,
    pub cause: FfiStr,
    pub owner: u64,
    pub world: u32,
}

#[repr(C)]
pub struct NotePlayEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub instrument: i32,
    pub pitch: i32,
}

#[repr(C)]
pub struct JukeboxChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub disc: i16,
    pub inserted: bool,
}

#[repr(C)]
pub struct LecternChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub book: i16,
    pub placed: bool,
}

#[repr(C)]
pub struct LecternPageEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub page: i32,
}

#[repr(C)]
pub struct ItemCookEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub station: FfiStr,
    pub input: i16,
    pub output: i16,
    pub world: u32,
}

#[repr(C)]
pub struct FuelBurnEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub station: FfiStr,
    pub fuel: i16,
    pub burn_ticks: u32,
    pub world: u32,
}

#[repr(C)]
pub struct BrewEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub ingredient: i16,
    pub world: u32,
}

#[repr(C)]
pub struct BeaconEffectEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub effect: i32,
    pub amplifier: i32,
    pub ticks: i32,
}

#[repr(C)]
pub struct LiquidFlowEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub liquid: u32,
    pub world: u32,
}

#[repr(C)]
pub struct SculkShriekEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub player: FfiStr,
    pub warning: u32,
    pub world: u32,
}

crate::event_data! {
    SignChangeEventData = 1000, "sign_change";
    BlockUseEventData = 1001, "block_use";
    BlockGrowEventData = 1002, "block_grow";
    TreeGrowEventData = 1003, "tree_grow";
    BlockFallEventData = 1004, "block_fall";
    BlockLandEventData = 1005, "block_land";
    BlockFormEventData = 1006, "block_form";
    BlockIgniteEventData = 1007, "block_ignite";
    TntPrimeEventData = 1008, "tnt_prime";
    NotePlayEventData = 1009, "note_play";
    JukeboxChangeEventData = 1010, "jukebox_change";
    LecternChangeEventData = 1011, "lectern_change";
    LecternPageEventData = 1012, "lectern_page";
    ItemCookEventData = 1013, "item_cook";
    FuelBurnEventData = 1014, "fuel_burn";
    BrewEventData = 1015, "brew";
    BeaconEffectEventData = 1016, "beacon_effect";
    LiquidFlowEventData = 1017, "liquid_flow";
    SculkShriekEventData = 1018, "sculk_shriek";
}
