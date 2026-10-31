use std::cell::Cell;

/**
An entity other than a player catches fire.

`event.cancel()` keeps the entity from burning.

Handled by [`Event::on_entity_combust`](crate::prelude::Event::on_entity_combust) and [`Server::on_entity_combust`](crate::server::Server::on_entity_combust).
*/
pub struct EntityCombustEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    pub(crate) ticks: Cell<u32>,
    /// What set it on fire, such as `sunlight`, `lava`, `fire` or `flame`.
    pub cause: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl EntityCombustEvent {
    /// Returns how long the entity burns, in ticks.
    pub fn ticks(&self) -> u32 {
        self.ticks.get()
    }

    /// Sets how long the entity burns, in ticks.
    pub fn set_ticks(&self, value: u32) {
        self.ticks.set(value);
    }
}
