/**
The redstone power of a block changes.

`event.cancel()` keeps the previous power.

Handled by [`Event::on_redstone_power`](crate::prelude::Event::on_redstone_power) and [`Server::on_redstone_power`](crate::server::Server::on_redstone_power).
*/
pub struct RedstonePowerEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the powered block.
    pub block: u32,
    /// Power before the change, from `0` to `15`.
    pub old_power: i32,
    /// Power after the change, from `0` to `15`.
    pub new_power: i32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
