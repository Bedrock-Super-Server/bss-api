/**
A minecart passes over a powered activator rail.

Returning `true` from a handler ignores the rail.

Handled by [`Event::on_minecart_activate`](crate::prelude::Event::on_minecart_activate) and [`Server::on_minecart_activate`](crate::server::Server::on_minecart_activate).
*/
pub struct MinecartActivateEvent {
    /// Runtime id of the minecart.
    pub entity: u64,
    /// Type of the minecart, such as `minecraft:tnt_minecart`.
    pub identifier: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the rider, or `0`.
    pub rider: u64,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
