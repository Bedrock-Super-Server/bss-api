/**
A piston extends or retracts.

Returning `true` from a handler keeps the piston still.

Handled by [`Event::on_piston_move`](crate::prelude::Event::on_piston_move) and [`Server::on_piston_move`](crate::server::Server::on_piston_move).
*/
pub struct PistonMoveEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// `true` when it extends, `false` when it retracts.
    pub extending: bool,
    /// Number of blocks moved.
    pub count: u32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
