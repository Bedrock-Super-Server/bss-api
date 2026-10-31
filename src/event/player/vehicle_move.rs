/**
A player moves the vehicle they ride.

`event.cancel()` keeps the vehicle where it was.

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

impl VehicleMoveEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
