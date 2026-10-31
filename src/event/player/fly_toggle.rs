/**
A player starts or stops flying, in creative or spectator mode or with the flight a plugin granted.

`event.cancel()` changes nothing.

Handled by [`Event::on_player_fly_toggle`](crate::prelude::Event::on_player_fly_toggle) and [`Server::on_player_fly_toggle`](crate::server::Server::on_player_fly_toggle).
*/
pub struct PlayerFlyToggleEvent {
    /// Name of the player.
    pub player: String,
    /// Whether the player flies now.
    pub flying: bool,
}

impl PlayerFlyToggleEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
