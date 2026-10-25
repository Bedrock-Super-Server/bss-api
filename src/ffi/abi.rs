use core::ffi::c_void;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerHandle(pub u64);

/// The item in a player's main hand, from [`Player::held`](crate::player::Player::held).
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HeldItem {
    /// Numeric id of the item, or `0` for an empty hand.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0`.
    pub block_runtime_id: u32,
    /// Size of the stack.
    pub count: u8,
    /// Damage of a tool, weapon or armor piece.
    pub damage: u16,
}

/// A plain stack of items in an inventory or a chest.
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ItemStack {
    /// Numeric id of the item, or `0` for an empty slot.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0`.
    pub block_runtime_id: u32,
    /// Size of the stack; `0` for an empty slot.
    pub count: u8,
    /// Damage of a tool, weapon or armor piece.
    pub damage: u16,
    /// Unique id the server gives each stack, to follow it in the item log.
    pub uid: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Uuid16(pub [u8; 16]);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FfiStr {
    pub ptr: *const u8,
    pub len: usize,
}

impl Default for FfiStr {
    fn default() -> Self {
        Self {
            ptr: core::ptr::null(),
            len: 0,
        }
    }
}

impl FfiStr {
    pub const fn new(s: &str) -> Self {
        Self {
            ptr: s.as_ptr(),
            len: s.len(),
        }
    }

    pub unsafe fn as_str(&self) -> Option<&str> {
        if self.ptr.is_null() {
            return None;
        }
        // SAFETY: guaranteed by the caller.
        let slice = unsafe { core::slice::from_raw_parts(self.ptr, self.len) };
        core::str::from_utf8(slice).ok()
    }

    pub unsafe fn as_bytes(&self) -> &[u8] {
        if self.ptr.is_null() {
            return &[];
        }
        // SAFETY: guaranteed by the caller.
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct FfiSlice<T> {
    pub ptr: *const T,
    pub len: usize,
}

pub type FormCallback = Option<unsafe extern "C" fn(response_json: FfiStr, user_data: *mut c_void)>;
/// Called with an event of a data-driven screen as JSON, as many times as the screen has events, and a last time when it closes.
pub type DduiCallback = Option<unsafe extern "C" fn(event_json: FfiStr, user_data: *mut c_void)>;
pub type HttpCallback = Option<unsafe extern "C" fn(answer_json: FfiStr, user_data: *mut c_void)>;
pub type InventoryCallback =
    Option<unsafe extern "C" fn(inventory: InventoryHandle, user_data: *mut c_void)>;
pub type VirtualChestCallback =
    Option<unsafe extern "C" fn(slot: usize, player: u64, user_data: *mut c_void)>;
pub type CommandCallback =
    Option<unsafe extern "C" fn(sender: FfiStr, args_json: FfiStr, user_data: *mut c_void)>;
pub type ServiceCallback =
    Option<unsafe extern "C" fn(request: FfiStr, user_data: *mut c_void) -> FfiStr>;
pub type GeneratorCallback =
    Option<unsafe extern "C" fn(cx: i32, cz: i32, chunk: ChunkHandle, user_data: *mut c_void)>;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ChunkHandle(pub *mut c_void);

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct InventoryHandle(pub *mut c_void);

// SAFETY: the handle is only dereferenced by the host, on its own thread, while the inventory
// callback that received it runs; sending or sharing the bare pointer value is harmless.
unsafe impl Send for InventoryHandle {}
// SAFETY: see `Send` above: the plugin never dereferences the handle itself.
unsafe impl Sync for InventoryHandle {}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct FormHandle(pub *mut c_void);
