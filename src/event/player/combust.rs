use std::cell::Cell;

/**
A player catches fire.

Returning `true` from a handler keeps the player from burning.

Handled by [`Event::on_player_combust`](crate::prelude::Event::on_player_combust) and [`Server::on_player_combust`](crate::server::Server::on_player_combust).
*/
pub struct PlayerCombustEvent {
    /// Name of the player.
    pub player: String,
    pub(crate) ticks: Cell<u32>,
}

impl PlayerCombustEvent {
    /// Returns how long the player burns, in ticks.
    pub fn ticks(&self) -> u32 {
        self.ticks.get()
    }

    /// Sets how long the player burns, in ticks.
    pub fn set_ticks(&self, value: u32) {
        self.ticks.set(value);
    }
}
