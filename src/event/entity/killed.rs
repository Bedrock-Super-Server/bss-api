use std::cell::Cell;

/**
An entity other than a player is killed.

`event.cancel()` keeps the entity alive, with one health point.

Handled by [`Event::on_entity_killed`](crate::prelude::Event::on_entity_killed) and [`Server::on_entity_killed`](crate::server::Server::on_entity_killed).
*/
pub struct EntityKilledEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// Runtime id of the killer, or `0` when the death has no killer.
    pub killer: u64,
    pub(crate) experience: Cell<u32>,
    pub(crate) drops: Cell<bool>,
}

impl EntityKilledEvent {
    /// Returns the experience the entity drops.
    pub fn experience(&self) -> u32 {
        self.experience.get()
    }

    /// Sets the experience the entity drops.
    pub fn set_experience(&self, value: u32) {
        self.experience.set(value);
    }

    /// Returns whether the entity drops its loot.
    pub fn drops(&self) -> bool {
        self.drops.get()
    }

    /// Sets whether the entity drops its loot.
    pub fn set_drops(&self, value: bool) {
        self.drops.set(value);
    }
}
