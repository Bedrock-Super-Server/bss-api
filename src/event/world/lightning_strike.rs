use std::cell::Cell;

/**
Lightning is about to strike.

Returning `true` from a handler cancels the strike.

Handled by [`Event::on_lightning_strike`](crate::prelude::Event::on_lightning_strike) and [`Server::on_lightning_strike`](crate::server::Server::on_lightning_strike).
*/
pub struct LightningStrikeEvent {
    pub(crate) x: Cell<f32>,
    pub(crate) y: Cell<f32>,
    pub(crate) z: Cell<f32>,
    /// Why, such as `weather` or `trident`.
    pub cause: String,
    pub(crate) fire: Cell<bool>,
    /// Runtime id of the entity responsible, such as a player with a Channeling trident, or `0`.
    pub owner: u64,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl LightningStrikeEvent {
    /// Returns the X coordinate of the strike.
    pub fn x(&self) -> f32 {
        self.x.get()
    }

    /// Sets the X coordinate of the strike.
    pub fn set_x(&self, value: f32) {
        self.x.set(value);
    }

    /// Returns the Y coordinate of the strike.
    pub fn y(&self) -> f32 {
        self.y.get()
    }

    /// Sets the Y coordinate of the strike.
    pub fn set_y(&self, value: f32) {
        self.y.set(value);
    }

    /// Returns the Z coordinate of the strike.
    pub fn z(&self) -> f32 {
        self.z.get()
    }

    /// Sets the Z coordinate of the strike.
    pub fn set_z(&self, value: f32) {
        self.z.set(value);
    }

    /// Returns whether the strike sets fire around it.
    pub fn fire(&self) -> bool {
        self.fire.get()
    }

    /// Sets whether the strike sets fire around it.
    pub fn set_fire(&self, value: bool) {
        self.fire.set(value);
    }
}
