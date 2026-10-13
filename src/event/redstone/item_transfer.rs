/**
A hopper moves an item from one container to another.

Returning `true` from a handler cancels the transfer.

Handled by [`Event::on_item_transfer`](crate::prelude::Event::on_item_transfer) and [`Server::on_item_transfer`](crate::server::Server::on_item_transfer).
*/
pub struct ItemTransferEvent {
    /// X coordinate of the source container.
    pub from_x: i32,
    /// Y coordinate of the source container.
    pub from_y: i32,
    /// Z coordinate of the source container.
    pub from_z: i32,
    /// X coordinate of the destination container.
    pub to_x: i32,
    /// Y coordinate of the destination container.
    pub to_y: i32,
    /// Z coordinate of the destination container.
    pub to_z: i32,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
