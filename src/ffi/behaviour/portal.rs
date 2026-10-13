use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct PortalCreateEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub width: u32,
    pub height: u32,
    pub axis: FfiStr,
    pub cause: FfiStr,
    pub player: FfiStr,
    pub world: u32,
}

#[repr(C)]
pub struct PlayerPortalEventData {
    pub header: EventHeader,
    pub player: FfiStr,
    pub portal: FfiStr,
    pub from_world: u32,
    pub to_world: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct EntityPortalEventData {
    pub header: EventHeader,
    pub entity: u64,
    pub identifier: FfiStr,
    pub portal: FfiStr,
    pub from_world: u32,
    pub to_world: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct EndGatewayEventData {
    pub header: EventHeader,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub world: u32,
}

crate::event_data! {
    PortalCreateEventData = 1041, "portal_create";
    PlayerPortalEventData = 1042, "player_portal";
    EntityPortalEventData = 1043, "entity_portal";
    EndGatewayEventData = 1044, "end_gateway";
}
