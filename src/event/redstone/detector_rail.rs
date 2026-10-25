/**
A minecart turns a detector rail on or off.

Returning `true` from a handler leaves the rail as it was.

Handled by [`Event::on_detector_rail`](crate::prelude::Event::on_detector_rail) and [`Server::on_detector_rail`](crate::server::Server::on_detector_rail).
*/
pub struct DetectorRailEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the minecart.
    pub entity: u64,
    /// `true` when the rail turns on, `false` when it turns off.
    pub active: bool,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
