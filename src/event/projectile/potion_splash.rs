use std::cell::Cell;

/**
A thrown potion breaks.

Returning `true` from a handler cancels the effect: the potion breaks without affecting anything.

Handled by [`Event::on_potion_splash`](crate::prelude::Event::on_potion_splash) and [`Server::on_potion_splash`](crate::server::Server::on_potion_splash).
*/
pub struct PotionSplashEvent {
    /// Runtime id of the potion.
    pub entity: u64,
    /// Id of the potion type.
    pub potion: i32,
    /// X coordinate, in blocks.
    pub x: f32,
    /// Y coordinate, in blocks.
    pub y: f32,
    /// Z coordinate, in blocks.
    pub z: f32,
    pub(crate) radius: Cell<f32>,
    /// `true` for a lingering potion, which leaves a cloud.
    pub lingering: bool,
}

impl PotionSplashEvent {
    /// Returns the radius of the area affected, in blocks.
    pub fn radius(&self) -> f32 {
        self.radius.get()
    }

    /// Sets the radius of the area affected, in blocks.
    pub fn set_radius(&self, value: f32) {
        self.radius.set(value);
    }
}
