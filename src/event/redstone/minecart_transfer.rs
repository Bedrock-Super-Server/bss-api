/**
A hopper moves an item into or out of a minecart.

Returning `true` from a handler cancels the transfer.

Handled by [`Event::on_minecart_transfer`](crate::prelude::Event::on_minecart_transfer) and [`Server::on_minecart_transfer`](crate::server::Server::on_minecart_transfer).
*/
pub struct MinecartTransferEvent {
    /// Runtime id of the minecart.
    pub entity: u64,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// `true` when the item goes into the minecart, `false` when it comes out.
    pub into_cart: bool,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
