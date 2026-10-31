use std::cell::Cell;

/**
A player's client asks for another view distance, in chunks. `radius` is what the server grants, within the limit the player has: the `view_distance` of the server, or the one a plugin set with `Player::set_view_distance`; set another one to grant that instead, within the same limit.

`event.cancel()` keeps the view distance the player has.

Handled by [`Event::on_player_view_distance_change`](crate::prelude::Event::on_player_view_distance_change) and [`Server::on_player_view_distance_change`](crate::server::Server::on_player_view_distance_change).
*/
pub struct PlayerViewDistanceChangeEvent {
    /// Name of the player.
    pub player: String,
    /// View distance the client asked for, in chunks.
    pub requested: i32,
    /// View distance the player had, in chunks.
    pub old_radius: i32,
    /// View distance the server grants, in chunks.
    pub(crate) radius: Cell<i32>,
}

impl PlayerViewDistanceChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl PlayerViewDistanceChangeEvent {
    /// Returns the current value of `radius`, which the handlers before this one may have changed.
    ///
    /// View distance the server grants, in chunks.
    pub fn radius(&self) -> i32 {
        self.radius.get()
    }

    /// Sets `radius`: the server uses the last value set.
    ///
    /// View distance the server grants, in chunks.
    pub fn set_radius(&self, value: i32) {
        self.radius.set(value);
    }
}
