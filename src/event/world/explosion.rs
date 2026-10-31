use std::cell::Cell;

/**
An explosion is about to happen.

`event.cancel()` cancels the explosion.

Handled by [`Event::on_explosion`](crate::prelude::Event::on_explosion) and [`Server::on_explosion`](crate::server::Server::on_explosion).
*/
pub struct ExplosionEvent {
    /// X coordinate of the centre.
    pub x: f32,
    /// Y coordinate of the centre.
    pub y: f32,
    /// Z coordinate of the centre.
    pub z: f32,
    pub(crate) power: Cell<f32>,
    pub(crate) fire: Cell<bool>,
    pub(crate) breaks_blocks: Cell<bool>,
    /// Runtime id of the entity that exploded or caused it, or `0`.
    pub owner: u64,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl ExplosionEvent {
    /// Returns the power of the explosion; TNT is `4.0`.
    pub fn power(&self) -> f32 {
        self.power.get()
    }

    /// Sets the power of the explosion; TNT is `4.0`.
    pub fn set_power(&self, value: f32) {
        self.power.set(value);
    }

    /// Returns whether the explosion sets fires.
    pub fn fire(&self) -> bool {
        self.fire.get()
    }

    /// Sets whether the explosion sets fires.
    pub fn set_fire(&self, value: bool) {
        self.fire.set(value);
    }

    /// Returns whether the explosion destroys blocks.
    pub fn breaks_blocks(&self) -> bool {
        self.breaks_blocks.get()
    }

    /// Sets whether the explosion destroys blocks.
    pub fn set_breaks_blocks(&self, value: bool) {
        self.breaks_blocks.set(value);
    }
}
