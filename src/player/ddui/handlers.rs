use parking_lot::Mutex;
use std::collections::HashMap;
use std::ffi::c_void;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, LazyLock};

use crate::ffi::abi::FfiStr;
use crate::ffi::host::HostApi;
use crate::player::Player;
use crate::player::ddui::{DduiEvent, DduiHandle};
use crate::registry::server;

pub(crate) type Handler = Arc<Mutex<dyn FnMut(&DduiHandle, DduiEvent) + Send>>;

/// The handler of each open screen, by the player's handle and the number of the screen.
static HANDLERS: LazyLock<Mutex<HashMap<(u64, u32), Handler>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub(crate) fn register(player: u64, form_id: u32, handler: Handler) {
    HANDLERS.lock().insert((player, form_id), handler);
}

pub(crate) fn is_open(player: u64, form_id: u32) -> bool {
    HANDLERS.lock().contains_key(&(player, form_id))
}

pub(crate) fn forget(player: u64, form_id: u32) {
    HANDLERS.lock().remove(&(player, form_id));
}

/// Takes every handler of a player's screens, which another screen is about to replace.
pub(crate) fn take_all(player: u64) -> Vec<(u32, Handler)> {
    let mut handlers = HANDLERS.lock();
    let ids: Vec<(u64, u32)> = handlers
        .keys()
        .filter(|(owner, _)| *owner == player)
        .copied()
        .collect();
    ids.into_iter()
        .filter_map(|key| handlers.remove(&key).map(|handler| (key.1, handler)))
        .collect()
}

/// The one function the server calls for every screen of every player: the screen is found in the
/// event, so the server keeps a single callback of this plugin and not one for each screen.
pub(crate) unsafe extern "C" fn answer(event_json: FfiStr, _user_data: *mut c_void) {
    // SAFETY: the event is valid for the duration of this call and copied here.
    let raw = unsafe { event_json.as_str() }.unwrap_or("").to_string();
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return;
    };
    deliver(server().host(), &value);
}

/// Hands an event of the server to the handler of its screen.
fn deliver(host: &'static HostApi, value: &serde_json::Value) {
    let (Some(player), Some(form_id)) = (value["handle"].as_u64(), value["form_id"].as_u64())
    else {
        return;
    };
    let form_id = form_id as u32;
    let Some(event) = DduiEvent::parse(value) else {
        return;
    };
    let closed = matches!(event, DduiEvent::Closed(_));
    let handler = if closed {
        HANDLERS.lock().remove(&(player, form_id))
    } else {
        HANDLERS.lock().get(&(player, form_id)).cloned()
    };
    let Some(handler) = handler else {
        return;
    };
    let handle = DduiHandle::new(Player::new(player, host), form_id);
    let _ = catch_unwind(AssertUnwindSafe(|| (handler.lock())(&handle, event)));
}
