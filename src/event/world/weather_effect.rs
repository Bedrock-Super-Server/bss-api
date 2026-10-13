use std::cell::Cell;

/**
The weather changes a block: snow piles up, water freezes, rain puts a fire out...

Returning `true` from a handler leaves the block as it is.

Handled by [`Event::on_weather_effect`](crate::prelude::Event::on_weather_effect) and [`Server::on_weather_effect`](crate::server::Server::on_weather_effect).
*/
pub struct WeatherEffectEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block before the change.
    pub block: u32,
    pub(crate) new_block: Cell<u32>,
    /// Why, such as `snow`, `freeze` or `extinguish`.
    pub cause: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl WeatherEffectEvent {
    /// Returns the runtime id of the block after the change.
    pub fn new_block(&self) -> u32 {
        self.new_block.get()
    }

    /// Sets the runtime id of the block after the change.
    pub fn set_new_block(&self, value: u32) {
        self.new_block.set(value);
    }
}
