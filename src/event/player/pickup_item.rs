/**
A player picks up an item from the ground.

Returning `true` from a handler leaves the item on the ground.

Handled by [`Event::on_pickup_item`](crate::prelude::Event::on_pickup_item).
*/
pub struct PickupItemEvent {
    /// Name of the player.
    pub name: String,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0` for an item that is not a block.
    pub block_runtime_id: u32,
    /// Number of items picked up.
    pub count: u8,
    /// Runtime id of the item entity.
    pub entity_id: u64,
}
