/**
A player drops an item from their inventory.

Returning `true` from a handler keeps the items in the inventory.

Handled by [`Event::on_drop_item`](crate::prelude::Event::on_drop_item).
*/
pub struct DropItemEvent {
    /// Name of the player.
    pub name: String,
    /// Numeric id of the item.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0` for an item that is not a block.
    pub block_runtime_id: u32,
    /// Number of items dropped.
    pub count: u8,
    /// Inventory slot the items come from.
    pub source_slot: u32,
}
