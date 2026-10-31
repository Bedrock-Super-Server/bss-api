/**
A player gets on or off a vehicle or a mob.

`event.cancel()` cancels it.

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

impl PlayerMountEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
