/**
A mob is about to spawn.

Returning `true` from a handler prevents the spawn.

Handled by [`Event::on_mob_spawn`](crate::prelude::Event::on_mob_spawn) and [`Server::on_mob_spawn`](crate::server::Server::on_mob_spawn).
*/
pub struct MobSpawnEvent {
    /// Type of the mob, such as `minecraft:zombie`.
    pub identifier: String,
    /// X coordinate, in blocks.
    pub x: f32,
    /// Y coordinate, in blocks.
    pub y: f32,
    /// Z coordinate, in blocks.
    pub z: f32,
    /// Why it spawns, such as `natural`, `spawner`, `patrol`, `build` (an iron golem, a snow golem or the wither) or `shrieker`.
    pub cause: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
