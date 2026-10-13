/**
An entity is removed from the world, whatever the reason.

Handled by [`Event::on_entity_despawn`](crate::prelude::Event::on_entity_despawn).
*/
pub struct EntityDespawnEvent {
    /// Runtime id of the entity.
    pub entity_id: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
}
