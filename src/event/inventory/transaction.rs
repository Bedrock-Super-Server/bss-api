use serde::{Deserialize, Serialize};

/// A slot of an inventory, as an inventory transaction sees it.
#[derive(Serialize, Deserialize, Clone)]
pub struct TransactionSlot {
    /// Container the slot belongs to, as the Bedrock protocol numbers it.
    pub container: u8,
    /// Index of the slot in the container.
    pub slot: u8,
    /// Numeric id of the item in the slot.
    pub item_id: i16,
    /// Number of items in the slot.
    pub count: u8,
}

/// One step of an [`InventoryTransactionEvent`].
#[derive(Serialize, Deserialize, Clone)]
pub enum TransactionAction {
    /// Items move from one slot to another.
    Move {
        /// Number of items moved.
        count: u8,
        /// Slot the items leave.
        source: TransactionSlot,
        /// Slot the items arrive in.
        destination: TransactionSlot,
    },
    /// Two slots exchange their contents.
    Swap {
        /// First slot.
        a: TransactionSlot,
        /// Second slot.
        b: TransactionSlot,
    },
    /// Items are dropped on the ground.
    Drop {
        /// Number of items dropped.
        count: u8,
        /// Slot the items leave.
        source: TransactionSlot,
    },
    /// Items are destroyed, as in the creative inventory bin.
    Destroy {
        /// Number of items destroyed.
        count: u8,
        /// Slot the items leave.
        source: TransactionSlot,
    },
    /// Items are used up by a crafting or another station.
    Consume {
        /// Number of items used up.
        count: u8,
        /// Slot the items leave.
        source: TransactionSlot,
    },
    /// The client reports a block it mined, to keep its prediction in step.
    Mine {
        /// Hotbar slot of the tool used.
        slot: u8,
    },
}

/**
A player moves, swaps, drops or uses items in an inventory.

`event.cancel()` rejects the whole request: the client is told to put everything back.

Handled by [`Event::on_inventory_transaction`](crate::prelude::Event::on_inventory_transaction).
*/
pub struct InventoryTransactionEvent {
    /// Name of the player.
    pub name: String,
    /// Id the client gave the request.
    pub request_id: i32,
    /// Number of actions in the request.
    pub action_count: u32,
    /// The actions, in the order the client sent them.
    pub actions: Vec<TransactionAction>,
}

impl InventoryTransactionEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
