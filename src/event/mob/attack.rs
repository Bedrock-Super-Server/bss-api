use std::cell::Cell;

/**
A mob attacks its target.

Returning `true` from a handler cancels the attack.

Handled by [`Event::on_mob_attack`](crate::prelude::Event::on_mob_attack) and [`Server::on_mob_attack`](crate::server::Server::on_mob_attack).
*/
pub struct MobAttackEvent {
    /// Runtime id of the attacking mob.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// Runtime id of the target.
    pub target: u64,
    pub(crate) damage: Cell<f32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl MobAttackEvent {
    /// Returns the damage dealt, in half-hearts.
    pub fn damage(&self) -> f32 {
        self.damage.get()
    }

    /// Sets the damage dealt, in half-hearts.
    pub fn set_damage(&self, value: f32) {
        self.damage.set(value);
    }
}
