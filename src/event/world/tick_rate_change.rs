use std::cell::Cell;

/**
The tick rate of the server is about to change, through `/tick` or a plugin.

`event.cancel()` keeps the current rate.

Handled by [`Event::on_tick_rate_change`](crate::prelude::Event::on_tick_rate_change) and [`Server::on_tick_rate_change`](crate::server::Server::on_tick_rate_change).
*/
pub struct TickRateChangeEvent {
    pub(crate) rate: Cell<f32>,
    pub(crate) frozen: Cell<bool>,
}

impl TickRateChangeEvent {
    /// Returns the new number of ticks per second.
    pub fn rate(&self) -> f32 {
        self.rate.get()
    }

    /// Sets the new number of ticks per second.
    pub fn set_rate(&self, value: f32) {
        self.rate.set(value);
    }

    /// Returns whether the game is frozen.
    pub fn frozen(&self) -> bool {
        self.frozen.get()
    }

    /// Sets whether the game is frozen.
    pub fn set_frozen(&self, value: bool) {
        self.frozen.set(value);
    }
}
