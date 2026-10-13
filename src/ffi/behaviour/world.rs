use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct ExplosionEventData {
    pub header: EventHeader,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub power: f32,
    pub fire: bool,
    pub breaks_blocks: bool,
    pub owner: u64,
    pub world: u32,
}

#[repr(C)]
pub struct ExplosionBlocksEventData {
    pub header: EventHeader,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub count: u32,
    pub drop_items: bool,
    pub world: u32,
}

#[repr(C)]
pub struct LightningStrikeEventData {
    pub header: EventHeader,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub cause: FfiStr,
    pub fire: bool,
    pub owner: u64,
    pub world: u32,
}

#[repr(C)]
pub struct RainChangeEventData {
    pub header: EventHeader,
    pub raining: bool,
    pub duration: u32,
    pub world: u32,
}

#[repr(C)]
pub struct ThunderChangeEventData {
    pub header: EventHeader,
    pub thundering: bool,
    pub duration: u32,
    pub world: u32,
}

#[repr(C)]
pub struct WeatherEffectEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: u32,
    pub new_block: u32,
    pub cause: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct TickRateChangeEventData {
    pub header: EventHeader,
    pub rate: f32,
    pub frozen: bool,
}

crate::event_data! {
    ExplosionEventData = 1019, "explosion";
    ExplosionBlocksEventData = 1020, "explosion_blocks";
    LightningStrikeEventData = 1021, "lightning_strike";
    RainChangeEventData = 1022, "rain_change";
    ThunderChangeEventData = 1023, "thunder_change";
    WeatherEffectEventData = 1024, "weather_effect";
    TickRateChangeEventData = 1025, "tick_rate_change";
}
