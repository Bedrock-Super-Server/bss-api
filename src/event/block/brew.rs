/**
A brewing stand finishes brewing.

Returning `true` from a handler cancels the brew.

Handled by [`Event::on_brew`](crate::prelude::Event::on_brew) and [`Server::on_brew`](crate::server::Server::on_brew).
*/
pub struct BrewEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Numeric id of the ingredient that was consumed.
    pub ingredient: i16,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
