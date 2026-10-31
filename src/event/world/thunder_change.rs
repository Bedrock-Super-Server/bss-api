use std::cell::Cell;

/**
A thunderstorm starts or stops.

`event.cancel()` keeps the current weather.

Handled by [`Event::on_thunder_change`](crate::prelude::Event::on_thunder_change) and [`Server::on_thunder_change`](crate::server::Server::on_thunder_change).
*/
pub struct ThunderChangeEvent {
    /// `true` when the storm starts, `false` when it stops.
    pub thundering: bool,
    pub(crate) duration: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl ThunderChangeEvent {
    /// Returns how long the new weather lasts, in ticks.
    pub fn duration(&self) -> u32 {
        self.duration.get()
    }

    /// Sets how long the new weather lasts, in ticks.
    pub fn set_duration(&self, value: u32) {
        self.duration.set(value);
    }
}
