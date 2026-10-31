/**
A hopper minecart picks up an item entity.

`event.cancel()` leaves the item where it is.

Handled by [`Event::on_minecart_pickup`](crate::prelude::Event::on_minecart_pickup) and [`Server::on_minecart_pickup`](crate::server::Server::on_minecart_pickup).
*/
pub struct MinecartPickupEvent {
    /// Runtime id of the minecart.
    pub entity: u64,
    /// Runtime id of the item entity.
    pub item_entity: u64,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
