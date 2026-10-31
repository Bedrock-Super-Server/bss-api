use std::cell::Cell;

/**
A player launches with a Riptide trident.

`event.cancel()` cancels the launch.

Handled by [`Event::on_trident_riptide`](crate::prelude::Event::on_trident_riptide) and [`Server::on_trident_riptide`](crate::server::Server::on_trident_riptide).
*/
pub struct TridentRiptideEvent {
    /// Name of the player.
    pub player: String,
    pub(crate) strength: Cell<f32>,
}

impl TridentRiptideEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl TridentRiptideEvent {
    /// Returns the launch strength.
    pub fn strength(&self) -> f32 {
        self.strength.get()
    }

    /// Sets the launch strength.
    pub fn set_strength(&self, value: f32) {
        self.strength.set(value);
    }
}
