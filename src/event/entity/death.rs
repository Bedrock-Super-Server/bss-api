/**
An entity dies.

Returning `true` from a handler keeps the entity alive.

Handled by [`Event::on_entity_death`](crate::prelude::Event::on_entity_death).
*/
pub struct EntityDeathEvent {
    /// Runtime id of the entity.
    pub entity_id: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
}
