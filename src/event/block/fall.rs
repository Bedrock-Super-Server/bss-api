/**
A block affected by gravity, such as sand or an anvil, starts to fall.

Returning `true` from a handler keeps the block in place.

Handled by [`Event::on_block_fall`](crate::prelude::Event::on_block_fall) and [`Server::on_block_fall`](crate::server::Server::on_block_fall).
*/
pub struct BlockFallEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the falling block.
    pub block: u32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
