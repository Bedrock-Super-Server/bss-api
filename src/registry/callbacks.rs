use parking_lot::Mutex;
use std::collections::HashMap;
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, LazyLock};

use crate::ffi::abi::{FfiStr, InventoryHandle};
use crate::player::Player;
use crate::player::inventory::Inventory;
use crate::registry::server;

type FormHandler = Box<dyn FnOnce(&str) + Send>;
type InventoryHandler = Box<dyn FnOnce(&Inventory) + Send>;
pub(crate) type ChestHandler = Arc<dyn Fn(usize, Player) + Send + Sync>;

static CHEST_HANDLERS: LazyLock<Mutex<HashMap<u64, ChestHandler>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn box_form(handler: FormHandler) -> *mut c_void {
    Box::into_raw(Box::new(handler)) as *mut c_void
}

pub(crate) fn box_inventory(handler: InventoryHandler) -> *mut c_void {
    Box::into_raw(Box::new(handler)) as *mut c_void
}

pub(crate) fn register_chest(player: u64, handler: ChestHandler) {
    CHEST_HANDLERS.lock().insert(player, handler);
}

pub(crate) unsafe extern "C" fn answer_form(response_json: FfiStr, user_data: *mut c_void) {
    // SAFETY: `user_data` is the `FormHandler` that `box_form` leaked for this form, and
    // the server calls this function at most once per form, so it is taken back once.
    let handler = unsafe { Box::from_raw(user_data as *mut FormHandler) };
    // SAFETY: the response is valid for the duration of this call and copied here.
    let raw = unsafe { response_json.as_str() }.unwrap_or("").to_string();
    let _ = catch_unwind(AssertUnwindSafe(move || (*handler)(&raw)));
}

pub(crate) unsafe extern "C" fn read_inventory(inventory: InventoryHandle, user_data: *mut c_void) {
    // SAFETY: `user_data` is the `InventoryHandler` that `box_inventory` leaked for this
    // request, and the server calls this function at most once per request.
    let handler = unsafe { Box::from_raw(user_data as *mut InventoryHandler) };
    let inv = Inventory::new(inventory, server().host());
    let _ = catch_unwind(AssertUnwindSafe(|| (*handler)(&inv)));
}

pub(crate) unsafe extern "C" fn click_chest(slot: usize, player: u64, _user_data: *mut c_void) {
    let handler = CHEST_HANDLERS.lock().get(&player).cloned();
    if let Some(handler) = handler {
        let player = Player::new(player, server().host());
        let _ = catch_unwind(AssertUnwindSafe(|| handler(slot, player)));
    }
}
