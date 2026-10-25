use std::cell::Cell;

/**
A player launches a projectile: an arrow, a trident, a firework, a fishing hook...

Returning `true` from a handler cancels the launch.

Handled by [`Event::on_projectile_launch`](crate::prelude::Event::on_projectile_launch) and [`Server::on_projectile_launch`](crate::server::Server::on_projectile_launch).
*/
pub struct ProjectileLaunchEvent {
    /// Name of the player.
    pub player: String,
    /// Type of the projectile, such as `minecraft:arrow` or `minecraft:thrown_trident`.
    pub identifier: String,
    pub(crate) force: Cell<f32>,
}

impl ProjectileLaunchEvent {
    /// Returns the launch speed.
    pub fn force(&self) -> f32 {
        self.force.get()
    }

    /// Sets the launch speed.
    pub fn set_force(&self, value: f32) {
        self.force.set(value);
    }
}
