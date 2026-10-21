pub mod callbacks;
pub mod events;
#[macro_use]
pub(crate) mod hook;
pub(crate) mod host_ptr;
pub(crate) mod macros;

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::OnceLock;

use crate::ffi::abi::FfiStr;
use crate::ffi::host::HostApi;
use crate::ffi::version::{ABI_MAJOR, HOST_MAGIC, PluginHeader, copy_sized};
use crate::ffi::vtable::PluginVtable;
use crate::registry::host_ptr::host_ref;
use crate::sdk::Plugin;
use crate::server::Server;

static HOST: OnceLock<usize> = OnceLock::new();
static PLUGIN: OnceLock<Box<dyn Plugin>> = OnceLock::new();

/**
Keeps the server's table of functions, the first time the server passes it. Returns whether it can
be used: its magic and major version are those of this crate.

The table is copied at the size this crate knows: a server of an older minor version built a
smaller table, and the functions it lacks read as absent instead of past its end.
*/
pub(crate) fn set_host(host: *const HostApi) -> bool {
    if HOST.get().is_some() {
        return true;
    }
    let header = host_ref(host).header;
    if header.magic != HOST_MAGIC || header.major != ABI_MAJOR {
        return false;
    }
    // SAFETY: the server's table holds `header.size` bytes and lives for the whole process, and
    // every field of `HostApi` accepts zero.
    let copy: HostApi = unsafe { copy_sized(host.cast(), header.size as usize) };
    let _ = HOST.set(Box::leak(Box::new(copy)) as *const HostApi as usize);
    true
}

/**
A string a plugin hands back to the server through an event field.

The server copies the field as soon as the event call returns, on the same thread, so the text only needs to live until then. The buffer is per thread: another thread dispatching the same event at the same time must not reallocate it while it is read.
*/
pub(crate) type HandBack = std::thread::LocalKey<std::cell::RefCell<String>>;

pub(crate) fn hand_back(buffer: &'static HandBack, text: String) -> FfiStr {
    /* The string is stored, not leaked: the next event of the same kind on
     * this thread replaces it, by which time the server has copied the
     * previous one. */
    buffer.with(|cell| {
        let mut held = cell.borrow_mut();
        *held = text;
        FfiStr {
            ptr: held.as_ptr(),
            len: held.len(),
        }
    })
}

pub(crate) fn server() -> Server {
    // SAFETY: `HOST` is only set by `on_enable`, from the server's `'static` vtable after
    // checking its ABI magic.
    unsafe { Server::from_raw(*HOST.get().expect("host not initialized") as *const HostApi) }
}

pub fn install(plugin: Box<dyn Plugin>) {
    let _ = PLUGIN.set(plugin);
}

/**
What `bss_plugin_describe` returns: builds the plugin the first time, with `make`, and returns its
table. A panic while the plugin is built or names itself returns null, which the server reports as
a plugin it cannot load, instead of unwinding out of an `extern "C"` function and aborting it.
*/
pub fn describe_with(make: impl FnOnce() -> Box<dyn Plugin>) -> *const PluginVtable {
    let described = catch_unwind(AssertUnwindSafe(|| {
        if PLUGIN.get().is_none() {
            install(make());
        }
        describe()
    }));
    described.unwrap_or_else(|_| {
        eprintln!("[bss-api] the plugin panicked while it was built; the server cannot load it");
        std::ptr::null()
    })
}

unsafe extern "C" fn on_enable(host: *const HostApi) {
    if !set_host(host) {
        eprintln!(
            "[bss-api] the server runs another major version of the plugin ABI than this plugin ({ABI_MAJOR}): rebuild the plugin for this server"
        );
        return;
    }
    if let Some(plugin) = PLUGIN.get() {
        let server = server();
        let _ = catch_unwind(AssertUnwindSafe(|| plugin.register(&server)));
    }
}

unsafe extern "C" fn on_disable(host: *const HostApi) {
    if !set_host(host) {
        return;
    }
    if let Some(plugin) = PLUGIN.get() {
        let server = server();
        let _ = catch_unwind(AssertUnwindSafe(|| plugin.disable(&server)));
    }
    crate::server::scheduler::clear();
}

/// The table of the plugin, built once and kept for as long as the library is loaded.
pub fn describe() -> *const PluginVtable {
    struct Shared(PluginVtable);
    // SAFETY: the table is built once and never changed; its raw pointers point at functions and
    // `'static` strings of this library.
    unsafe impl Sync for Shared {}
    // SAFETY: see `Sync`.
    unsafe impl Send for Shared {}
    static TABLE: OnceLock<Shared> = OnceLock::new();
    &TABLE.get_or_init(|| Shared(vtable())).0
}

fn vtable() -> PluginVtable {
    let (name, version) = PLUGIN
        .get()
        .map(|plugin| (plugin.name(), plugin.version()))
        .unwrap_or(("plugin", "0.0.0"));
    PluginVtable {
        header: PluginHeader::of::<PluginVtable>(),
        name: FfiStr::new(name),
        version: FfiStr::new(version),
        on_enable: Some(on_enable),
        on_disable: Some(on_disable),
    }
}
