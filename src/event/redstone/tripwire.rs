/**
A tripwire is triggered or released.

Returning `true` from a handler leaves the tripwire as it was.

Handled by [`Event::on_tripwire`](crate::prelude::Event::on_tripwire) and [`Server::on_tripwire`](crate::server::Server::on_tripwire).
*/
pub struct TripwireEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// `true` when the tripwire is triggered, `false` when it is released.
    pub powered: bool,
    /// Name of the player who triggered it, or an empty string.
    pub player: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
