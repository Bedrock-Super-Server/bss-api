use std::cell::Cell;

/**
A sculk sensor detects a vibration.

`event.cancel()` keeps the sensor from reacting.

Handled by [`Event::on_sculk_vibration`](crate::prelude::Event::on_sculk_vibration) and [`Server::on_sculk_vibration`](crate::server::Server::on_sculk_vibration).
*/
pub struct SculkVibrationEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// X coordinate of the vibration.
    pub source_x: i32,
    /// Y coordinate of the vibration.
    pub source_y: i32,
    /// Z coordinate of the vibration.
    pub source_z: i32,
    pub(crate) frequency: Cell<i32>,
    pub(crate) power: Cell<i32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl SculkVibrationEvent {
    /// Returns the frequency of the vibration, from `1` to `15`.
    pub fn frequency(&self) -> i32 {
        self.frequency.get()
    }

    /// Sets the frequency of the vibration, from `1` to `15`.
    pub fn set_frequency(&self, value: i32) {
        self.frequency.set(value);
    }

    /// Returns the redstone signal the sensor emits, from `0` to `15`.
    pub fn power(&self) -> i32 {
        self.power.get()
    }

    /// Sets the redstone signal the sensor emits, from `0` to `15`.
    pub fn set_power(&self, value: i32) {
        self.power.set(value);
    }
}
