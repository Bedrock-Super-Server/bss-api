/**
A player gets on or off a vehicle or a mob.

Returning `true` from a handler cancels it.

Handled by [`Event::on_player_mount`](crate::prelude::Event::on_player_mount) and [`Server::on_player_mount`](crate::server::Server::on_player_mount).
*/
pub struct PlayerMountEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the vehicle.
    pub vehicle: u64,
    /// `true` when the player gets on, `false` when they get off.
    pub mounted: bool,
}
