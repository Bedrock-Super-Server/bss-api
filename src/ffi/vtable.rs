use crate::ffi::abi::FfiStr;
use crate::ffi::host::HostApi;
use crate::ffi::version::PluginHeader;

/**
What a plugin tells the server about itself.

Events are not part of it: a plugin subscribes to the ones it wants from `on_enable`, with
[`HostApi::event_subscribe`](crate::ffi::HostApi::event_subscribe). Fields are only ever added at
the end, and the server reads them up to `header.size`.
*/
#[repr(C)]
pub struct PluginVtable {
    /// Size and version of this table: fields past `header.size` read as absent.
    pub header: PluginHeader,
    /// Name shown in the server log; plugins are told apart by it.
    pub name: FfiStr,
    /// Version shown in the server log.
    pub version: FfiStr,
    /// Called once the plugin is loaded: subscribe to events and register commands here.
    pub on_enable: Option<unsafe extern "C" fn(host: *const HostApi)>,
    /// Called once when the plugin is unloaded or the server stops.
    pub on_disable: Option<unsafe extern "C" fn(host: *const HostApi)>,
}

/// The function a plugin exports as `bss_plugin_describe`: the address of its table, which stays valid while the library is loaded.
pub type PluginDescribe = unsafe extern "C" fn() -> *const PluginVtable;

/// The symbol under which a plugin exports its [`PluginDescribe`] function.
pub const DESCRIBE_SYMBOL: &[u8] = b"bss_plugin_describe";
