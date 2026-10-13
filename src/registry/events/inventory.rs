use crate::event::inventory::{InventoryCloseEvent, InventoryOpenEvent, InventoryTransactionEvent};
use crate::ffi::events::*;
use crate::registry::macros::{event_hook, event_hook_cancel};
use crate::server::{Server, ffi_string};

event_hook_cancel!(
    on_inventory_open,
    INVENTORY_OPEN,
    open,
    InventoryOpenEventData,
    InventoryOpenEvent,
    |e: &InventoryOpenEventData| InventoryOpenEvent {
        name: ffi_string(e.name),
        window_id: e.window_id,
        window_type: e.window_type,
    }
);

event_hook!(
    on_inventory_close,
    INVENTORY_CLOSE,
    close,
    InventoryCloseEventData,
    InventoryCloseEvent,
    |e: &InventoryCloseEventData| {
        InventoryCloseEvent {
            name: ffi_string(e.name),
            window_id: e.window_id,
            window_type: e.window_type,
            server: e.server,
        }
    }
);

event_hook_cancel!(
    on_inventory_transaction,
    INVENTORY_TX,
    transaction,
    InventoryTransactionEventData,
    InventoryTransactionEvent,
    |e: &InventoryTransactionEventData| {
        InventoryTransactionEvent {
            name: ffi_string(e.name),
            request_id: e.request_id,
            action_count: e.action_count,
            actions: serde_json::from_str(&ffi_string(e.actions_json)).unwrap_or_default(),
        }
    }
);
