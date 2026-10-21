use crate::ffi::abi::FfiStr;
use crate::ffi::event::EventHeader;

#[repr(C)]
pub struct ServerStartedEventData {
    pub header: EventHeader,
    pub worlds: u32,
}

#[repr(C)]
pub struct ServerStopEventData {
    pub header: EventHeader,
    pub players: u32,
}

#[repr(C)]
pub struct ConsoleCommandEventData {
    pub header: EventHeader,
    pub command: FfiStr,
}

#[repr(C)]
pub struct PluginEnableEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub version: FfiStr,
}

#[repr(C)]
pub struct PluginDisableEventData {
    pub header: EventHeader,
    pub name: FfiStr,
    pub version: FfiStr,
}

crate::event_data! {
    ServerStartedEventData = 1106, "server_started";
    ServerStopEventData = 1107, "server_stop";
    ConsoleCommandEventData = 1108, "console_command";
    PluginEnableEventData = 1109, "plugin_enable";
    PluginDisableEventData = 1110, "plugin_disable";
}
