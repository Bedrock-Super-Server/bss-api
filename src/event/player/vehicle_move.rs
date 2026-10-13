/**
A player moves the vehicle they ride.

Returning `true` from a handler keeps the vehicle where it was.

Handled by [`Event::on_vehicle_move`](crate::prelude::Event::on_vehicle_move) and [`Server::on_vehicle_move`](crate::server::Server::on_vehicle_move).
*/
pub struct VehicleMoveEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the vehicle.
    pub vehicle: u64,
    /// X coordinate the vehicle moves to.
    pub x: f32,
    /// Y coordinate the vehicle moves to.
    pub y: f32,
    /// Z coordinate the vehicle moves to.
    pub z: f32,
}
