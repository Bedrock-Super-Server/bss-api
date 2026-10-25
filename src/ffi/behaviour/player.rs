use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct PlayerDamageEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub source: FfiStr,
    pub damage: f32,
    pub attacker: FfiStr,
}

#[repr(C)]
pub struct PlayerHealEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub amount: f32,
    pub reason: FfiStr,
}

#[repr(C)]
pub struct PlayerCombustEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub ticks: u32,
}

#[repr(C)]
pub struct FoodChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub old_food: i32,
    pub food: i32,
    pub saturation: f32,
}

#[repr(C)]
pub struct ExperienceChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub level: i32,
    pub total: i32,
    pub amount: i32,
}

#[repr(C)]
pub struct ExperiencePickupEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub amount: u32,
}

#[repr(C)]
pub struct ItemMendEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub repair: u32,
}

#[repr(C)]
pub struct ItemDamageEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub slot: u32,
    pub damage: u32,
}

#[repr(C)]
pub struct ItemBreakEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub slot: u32,
}

#[repr(C)]
pub struct ShieldBlockEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub damage: f32,
    pub attacker: FfiStr,
}

#[repr(C)]
pub struct TotemUseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
}

#[repr(C)]
pub struct PlayerWarpEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub cause: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct PlayerMountEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub vehicle: u64,
    pub mounted: bool,
}

#[repr(C)]
pub struct VehicleMoveEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub vehicle: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct CrossbowLoadEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub ammo: i16,
}

#[repr(C)]
pub struct TridentRiptideEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub strength: f32,
}

#[repr(C)]
pub struct FishEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub action: FfiStr,
    pub item: i16,
    pub experience: u32,
    pub hooked: u64,
}

#[repr(C)]
pub struct HornBlowEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub variant: i32,
}

#[repr(C)]
pub struct BookEditEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub slot: u32,
    pub action: FfiStr,
    pub page: i32,
    pub text: FfiStr,
}

#[repr(C)]
pub struct OffhandChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub item: i16,
    pub count: u32,
}

#[repr(C)]
pub struct PlayerBedEnterEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub world: u32,
}

#[repr(C)]
pub struct PlayerBedLeaveEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub cause: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct PlayerViewDistanceChangeEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub requested: i32,
    pub old_radius: i32,
    pub radius: i32,
}

#[repr(C)]
pub struct PlayerKickEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub reason: FfiStr,
}

#[repr(C)]
pub struct PlayerJumpEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct PlayerEffectAddEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub effect: FfiStr,
    pub amplifier: u32,
    pub duration: i32,
}

#[repr(C)]
pub struct PlayerEffectRemoveEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub effect: FfiStr,
    pub amplifier: u32,
}

#[repr(C)]
pub struct DduiResponseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub form_id: u32,
    pub element: FfiStr,
    pub kind: FfiStr,
    pub number: i32,
    pub flag: bool,
    pub text: FfiStr,
}

#[repr(C)]
pub struct DduiCloseEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub form_id: u32,
    pub reason: FfiStr,
}

crate::event_data! {
    PlayerDamageEventData = 1048, "player_damage";
    PlayerHealEventData = 1049, "player_heal";
    PlayerCombustEventData = 1050, "player_combust";
    FoodChangeEventData = 1051, "food_change";
    ExperienceChangeEventData = 1052, "experience_change";
    ExperiencePickupEventData = 1053, "experience_pickup";
    ItemMendEventData = 1054, "item_mend";
    ItemDamageEventData = 1055, "item_damage";
    ItemBreakEventData = 1056, "item_break";
    ShieldBlockEventData = 1057, "shield_block";
    TotemUseEventData = 1058, "totem_use";
    PlayerWarpEventData = 1059, "player_warp";
    PlayerMountEventData = 1060, "player_mount";
    VehicleMoveEventData = 1061, "vehicle_move";
    CrossbowLoadEventData = 1062, "crossbow_load";
    TridentRiptideEventData = 1063, "trident_riptide";
    FishEventData = 1064, "fish";
    HornBlowEventData = 1065, "horn_blow";
    BookEditEventData = 1066, "book_edit";
    OffhandChangeEventData = 1067, "offhand_change";
    PlayerBedEnterEventData = 1099, "player_bed_enter";
    PlayerBedLeaveEventData = 1100, "player_bed_leave";
    PlayerViewDistanceChangeEventData = 1101, "player_view_distance_change";
    PlayerKickEventData = 1102, "player_kick";
    PlayerJumpEventData = 1103, "player_jump";
    PlayerEffectAddEventData = 1104, "player_effect_add";
    PlayerEffectRemoveEventData = 1105, "player_effect_remove";
    DduiResponseEventData = 1111, "ddui_response";
    DduiCloseEventData = 1112, "ddui_close";
}
