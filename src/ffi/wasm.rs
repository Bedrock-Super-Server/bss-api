/*!
The side of a plugin (built to `wasm32-wasip1`) that talks to the server.

A plugin lives in its own memory, in a sandbox, and cannot be handed the server's functions as
pointers. Its [`HostApi`] is built here from the functions the module imports from `bss`, which
the server provides with the same signatures, reading and writing the plugin's memory itself. The
rest of this crate sees a table of functions as any other.

The server drives the plugin through the functions [`export_plugin!`](crate::export_plugin)
exports on this target: it enables and disables it, writes the text and events it hands over into
memory the plugin allocates, and calls the plugin's callbacks, which are indices in the plugin's
function table, through the `bss_call_*` trampolines.
*/

use core::ffi::c_void;
use std::alloc::Layout;
use std::cell::RefCell;
use std::sync::OnceLock;

use crate::ffi::abi::*;
use crate::ffi::event::{EventCallback, EventHeader};
use crate::ffi::host::{EntityData, HostApi, VirtualSlot};
use crate::ffi::vtable::PluginVtable;

mod imports {
    #![allow(clippy::too_many_arguments)]
    use crate::ffi::abi::*;
    use crate::ffi::host::{BlockEdit, EntityData, HostApi, VirtualSlot};
    use core::ffi::c_void;

    include!(concat!(env!("OUT_DIR"), "/wasm_host.rs"));
}

#[link(wasm_import_module = "bss")]
unsafe extern "C" {
    /// Why the server does not provide the function `name` to this plugin, as a number: `0` when it
    /// does, `1` when it has no such function, `2` when the plugin's section of `plugins.toml`
    /// denies it, and `3` plus the index in [`GRANTS`](crate::ffi::GRANTS) of the grant it needs.
    #[link_name = "__refusal"]
    fn refusal(name: *const u8, len: usize) -> u32;
}

/// Why the server does not provide the function `name`: see `__refusal`.
pub(crate) fn refusal_of(name: &str) -> u32 {
    // SAFETY: the name is a live `&str` for the duration of the call.
    unsafe { refusal(name.as_ptr(), name.len()) }
}

/// Version of the glue between the server and a WebAssembly plugin, which the server checks.
pub const GLUE_VERSION: u32 = 1;

static TABLE: OnceLock<HostApi> = OnceLock::new();

/// The plugin's table of the server's functions.
pub fn host() -> &'static HostApi {
    TABLE.get_or_init(|| imports::table(|name| refusal_of(name) == 0))
}

/// Calls `on_enable` of the plugin's table, with the server's functions.
///
/// # Safety
///
/// `vtable` is the table `bss_plugin_describe` returned.
pub unsafe fn enable(vtable: *const PluginVtable) {
    // SAFETY: guaranteed by the caller.
    if let Some(on_enable) = unsafe { vtable.as_ref() }.and_then(|vtable| vtable.on_enable) {
        // SAFETY: `host()` is a `'static` table of functions that accept any argument.
        unsafe { on_enable(host()) };
    }
}

/// Calls `on_disable` of the plugin's table.
///
/// # Safety
///
/// `vtable` is the table `bss_plugin_describe` returned.
pub unsafe fn disable(vtable: *const PluginVtable) {
    // SAFETY: guaranteed by the caller.
    if let Some(on_disable) = unsafe { vtable.as_ref() }.and_then(|vtable| vtable.on_disable) {
        // SAFETY: see `enable`.
        unsafe { on_disable(host()) };
    }
}

thread_local! {
    /// What the server allocated in this plugin since it last entered it from outside.
    static SCRATCH: RefCell<Vec<(usize, Layout)>> = const { RefCell::new(Vec::new()) };
}

/// Memory for the server to write text or an event into, freed by the next [`reset`]. Returns 0
/// when there is none.
pub fn alloc(size: u32, align: u32) -> u32 {
    let align = (align.max(1) as usize).next_power_of_two();
    let Ok(layout) = Layout::from_size_align(size.max(1) as usize, align) else {
        return 0;
    };
    // SAFETY: the layout has a nonzero size.
    let ptr = unsafe { std::alloc::alloc_zeroed(layout) };
    if ptr.is_null() {
        return 0;
    }
    SCRATCH.with(|scratch| scratch.borrow_mut().push((ptr as usize, layout)));
    ptr as u32
}

/// Frees what the server allocated: it calls this before it enters the plugin from outside, when
/// nothing of what it handed over before is in use any more.
pub fn reset() {
    SCRATCH.with(|scratch| {
        for (ptr, layout) in scratch.borrow_mut().drain(..) {
            // SAFETY: `ptr` was allocated by `alloc` with this layout and is freed once.
            unsafe { std::alloc::dealloc(ptr as *mut u8, layout) };
        }
    });
}

/// The function at `index` of the plugin's table, as a pointer of type `F`.
///
/// # Safety
///
/// `F` is a function pointer type. Calling it with another signature than the one the function
/// has traps, which the server reports, so a wrong index cannot reach memory.
unsafe fn function<F: Copy>(index: u32) -> Option<F> {
    const { assert!(core::mem::size_of::<F>() == core::mem::size_of::<usize>()) };
    // SAFETY: on wasm32 a function pointer is its index in the table, a `usize`.
    (index != 0).then(|| unsafe { core::mem::transmute_copy(&(index as usize)) })
}

/// Calls an [`EventCallback`] with an event the server wrote at `event`.
///
/// # Safety
///
/// `event` is the address of the event data the server wrote, for the duration of the call.
pub unsafe fn call_event(callback: u32, event: u32, user_data: u32) {
    // SAFETY: see `function`.
    if let Some(callback) = unsafe { function::<EventCallback>(callback) } {
        // SAFETY: the server wrote the event at `event`; `user_data` is what the plugin subscribed with.
        unsafe { callback(host(), event as *mut EventHeader, user_data as *mut c_void) };
    }
}

/// Reads the [`FfiStr`] the server wrote at `at`.
///
/// # Safety
///
/// The server wrote an `FfiStr` at `at`.
unsafe fn text_at(at: u32) -> FfiStr {
    if at == 0 {
        return FfiStr::default();
    }
    // SAFETY: guaranteed by the caller.
    unsafe { *(at as *const FfiStr) }
}

/// Calls a [`FormCallback`] with the answer the server wrote at `response`.
///
/// # Safety
///
/// `response` is 0 or holds an [`FfiStr`] the server wrote.
pub unsafe fn call_form(callback: u32, response: u32, user_data: u32) {
    type F = unsafe extern "C" fn(FfiStr, *mut c_void);
    // SAFETY: see `function` and `text_at`.
    unsafe {
        if let Some(callback) = function::<F>(callback) {
            callback(text_at(response), user_data as *mut c_void);
        }
    }
}

/// Calls an [`InventoryCallback`] with the inventory the server numbered `inventory`.
///
/// # Safety
///
/// See `function`.
pub unsafe fn call_inventory(callback: u32, inventory: u32, user_data: u32) {
    type F = unsafe extern "C" fn(InventoryHandle, *mut c_void);
    // SAFETY: see `function`; the handle is a number only the server reads.
    unsafe {
        if let Some(callback) = function::<F>(callback) {
            callback(
                InventoryHandle(inventory as usize as *mut c_void),
                user_data as *mut c_void,
            );
        }
    }
}

/// Calls a [`VirtualChestCallback`].
///
/// # Safety
///
/// See `function`.
pub unsafe fn call_chest(callback: u32, slot: u32, player: u64, user_data: u32) {
    type F = unsafe extern "C" fn(usize, u64, *mut c_void);
    // SAFETY: see `function`.
    unsafe {
        if let Some(callback) = function::<F>(callback) {
            callback(slot as usize, player, user_data as *mut c_void);
        }
    }
}

/// Calls a [`CommandCallback`] with the sender and arguments the server wrote.
///
/// # Safety
///
/// See `function` and `text_at`.
pub unsafe fn call_command(callback: u32, sender: u32, args: u32, user_data: u32) {
    type F = unsafe extern "C" fn(FfiStr, FfiStr, *mut c_void);
    // SAFETY: see `function` and `text_at`.
    unsafe {
        if let Some(callback) = function::<F>(callback) {
            callback(text_at(sender), text_at(args), user_data as *mut c_void);
        }
    }
}

/// Calls a [`ServiceCallback`] with the request the server wrote, and writes its answer at `out`.
///
/// # Safety
///
/// See `function` and `text_at`; `out` is room for an [`FfiStr`].
pub unsafe fn call_service(callback: u32, request: u32, user_data: u32, out: u32) {
    type F = unsafe extern "C" fn(FfiStr, *mut c_void) -> FfiStr;
    // SAFETY: see `function` and `text_at`; the server allocated `out`.
    unsafe {
        let answer = match function::<F>(callback) {
            Some(callback) => callback(text_at(request), user_data as *mut c_void),
            None => FfiStr::default(),
        };
        if out != 0 {
            *(out as *mut FfiStr) = answer;
        }
    }
}

/// Calls a [`GeneratorCallback`] with the chunk the server numbered `chunk`.
///
/// # Safety
///
/// See `function`.
pub unsafe fn call_generator(callback: u32, cx: i32, cz: i32, chunk: u32, user_data: u32) {
    type F = unsafe extern "C" fn(i32, i32, ChunkHandle, *mut c_void);
    // SAFETY: see `function`; the handle is a number only the server reads.
    unsafe {
        if let Some(callback) = function::<F>(callback) {
            callback(
                cx,
                cz,
                ChunkHandle(chunk as usize as *mut c_void),
                user_data as *mut c_void,
            );
        }
    }
}

// The server writes and reads these types in the plugin's memory with this layout.
const _: () = {
    use core::mem::{offset_of, size_of};
    assert!(size_of::<FfiStr>() == 8);
    assert!(size_of::<ItemStack>() == 24 && offset_of!(ItemStack, uid) == 16);
    assert!(size_of::<HeldItem>() == 12 && offset_of!(HeldItem, damage) == 10);
    assert!(size_of::<EntityData>() == 56);
    assert!(offset_of!(EntityData, nametag) == 36 && offset_of!(EntityData, invisible) == 50);
    assert!(size_of::<VirtualSlot>() == 28 && offset_of!(VirtualSlot, lore_json) == 20);
    assert!(size_of::<EventHeader>() == 16);
    assert!(offset_of!(PluginVtable, name) == 8 && offset_of!(PluginVtable, on_enable) == 24);
};
