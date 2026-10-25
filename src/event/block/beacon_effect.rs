use std::cell::Cell;

/**
A beacon is about to give its effect to a player in range.

Returning `true` from a handler withholds the effect this time.

Handled by [`Event::on_beacon_effect`](crate::prelude::Event::on_beacon_effect) and [`Server::on_beacon_effect`](crate::server::Server::on_beacon_effect).
*/
pub struct BeaconEffectEvent {
    /// Name of the player receiving the effect.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) effect: Cell<i32>,
    pub(crate) amplifier: Cell<i32>,
    pub(crate) ticks: Cell<i32>,
}

impl BeaconEffectEvent {
    /// Returns the id of the effect given.
    pub fn effect(&self) -> i32 {
        self.effect.get()
    }

    /// Sets the id of the effect given.
    pub fn set_effect(&self, value: i32) {
        self.effect.set(value);
    }

    /// Returns the amplifier of the effect, where `0` is level I.
    pub fn amplifier(&self) -> i32 {
        self.amplifier.get()
    }

    /// Sets the amplifier of the effect, where `0` is level I.
    pub fn set_amplifier(&self, value: i32) {
        self.amplifier.set(value);
    }

    /// Returns how long the effect lasts, in ticks.
    pub fn ticks(&self) -> i32 {
        self.ticks.get()
    }

    /// Sets how long the effect lasts, in ticks.
    pub fn set_ticks(&self, value: i32) {
        self.ticks.set(value);
    }
}
