/**
An entity appears in the world, whatever the reason.

Handled by [`Event::on_entity_spawn`](crate::prelude::Event::on_entity_spawn).
*/
pub struct EntitySpawnEvent {
    /// Runtime id of the entity.
    pub entity_id: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// X coordinate, in blocks.
    pub x: f32,
    /// Y coordinate, in blocks.
    pub y: f32,
    /// Z coordinate, in blocks.
    pub z: f32,
    /// Id of the world, in load order: the default world is `0`.
    pub world_id: u32,
}
