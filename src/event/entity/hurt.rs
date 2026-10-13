use std::cell::Cell;

/**
An entity other than a player takes damage from any source.

Returning `true` from a handler cancels the damage.

Handled by [`Event::on_entity_hurt`](crate::prelude::Event::on_entity_hurt) and [`Server::on_entity_hurt`](crate::server::Server::on_entity_hurt).
*/
pub struct EntityHurtEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    pub(crate) damage: Cell<f32>,
    /// Runtime id of the entity that caused the damage, or `0`.
    pub attacker: u64,
}

impl EntityHurtEvent {
    /// Returns the damage taken, in half-hearts.
    pub fn damage(&self) -> f32 {
        self.damage.get()
    }

    /// Sets the damage taken, in half-hearts.
    pub fn set_damage(&self, value: f32) {
        self.damage.set(value);
    }
}
