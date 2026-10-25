use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct ProjectileLaunchEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub identifier: FfiStr,
    pub force: f32,
}

#[repr(C)]
pub struct ProjectileHitEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub owner: u64,
    pub target: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub damage: f32,
}

#[repr(C)]
pub struct ProjectilePickupEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub entity: u64,
    pub item: i16,
}

#[repr(C)]
pub struct PotionSplashEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub potion: i32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub lingering: bool,
}

crate::event_data! {
    ProjectileLaunchEventData = 1074, "projectile_launch";
    ProjectileHitEventData = 1075, "projectile_hit";
    ProjectilePickupEventData = 1076, "projectile_pickup";
    PotionSplashEventData = 1077, "potion_splash";
}
