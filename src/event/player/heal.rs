use std::cell::Cell;

/**
A player regains health.

`event.cancel()` cancels the healing.

Handled by [`Event::on_player_heal`](crate::prelude::Event::on_player_heal) and [`Server::on_player_heal`](crate::server::Server::on_player_heal).
*/
pub struct PlayerHealEvent {
    /// Name of the player.
    pub player: String,
    pub(crate) amount: Cell<f32>,
    /// Why, such as `effect` for regeneration or `saturation` for a full hunger bar.
    pub reason: String,
}

impl PlayerHealEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl PlayerHealEvent {
    /// Returns the health regained, in half-hearts.
    pub fn amount(&self) -> f32 {
        self.amount.get()
    }

    /// Sets the health regained, in half-hearts.
    pub fn set_amount(&self, value: f32) {
        self.amount.set(value);
    }
}
