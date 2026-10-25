/**
A hopper picks up an item entity lying above it.

Returning `true` from a handler leaves the item where it is.

Handled by [`Event::on_hopper_pickup`](crate::prelude::Event::on_hopper_pickup) and [`Server::on_hopper_pickup`](crate::server::Server::on_hopper_pickup).
*/
pub struct HopperPickupEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the item entity.
    pub entity: u64,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
