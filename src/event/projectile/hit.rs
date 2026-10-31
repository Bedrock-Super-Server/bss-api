use std::cell::Cell;

/**
A projectile hits a block or an entity.

`event.cancel()` cancels the hit.

Handled by [`Event::on_projectile_hit`](crate::prelude::Event::on_projectile_hit) and [`Server::on_projectile_hit`](crate::server::Server::on_projectile_hit).
*/
pub struct ProjectileHitEvent {
    /// Runtime id of the projectile.
    pub entity: u64,
    /// Type of the projectile, such as `minecraft:arrow`.
    pub identifier: String,
    /// Runtime id of the entity that shot it, or `0`.
    pub owner: u64,
    /// Runtime id of the entity hit, or `0` when a block was hit.
    pub target: u64,
    /// X coordinate of the impact.
    pub x: f32,
    /// Y coordinate of the impact.
    pub y: f32,
    /// Z coordinate of the impact.
    pub z: f32,
    pub(crate) damage: Cell<f32>,
}

impl ProjectileHitEvent {
    /// Returns the damage dealt to the entity hit, in half-hearts.
    pub fn damage(&self) -> f32 {
        self.damage.get()
    }

    /// Sets the damage dealt to the entity hit, in half-hearts.
    pub fn set_damage(&self, value: f32) {
        self.damage.set(value);
    }
}
