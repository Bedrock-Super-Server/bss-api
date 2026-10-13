use std::cell::Cell;

/**
Rain starts or stops.

Returning `true` from a handler keeps the current weather.

Handled by [`Event::on_rain_change`](crate::prelude::Event::on_rain_change) and [`Server::on_rain_change`](crate::server::Server::on_rain_change).
*/
pub struct RainChangeEvent {
    /// `true` when rain starts, `false` when it stops.
    pub raining: bool,
    pub(crate) duration: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl RainChangeEvent {
    /// Returns how long the new weather lasts, in ticks.
    pub fn duration(&self) -> u32 {
        self.duration.get()
    }

    /// Sets how long the new weather lasts, in ticks.
    pub fn set_duration(&self, value: u32) {
        self.duration.set(value);
    }
}
