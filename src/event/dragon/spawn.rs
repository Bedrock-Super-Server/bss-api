/**
An ender dragon is about to spawn in the End.

`event.cancel()` prevents the dragon from spawning.

Handled by [`Event::on_dragon_spawn`](crate::prelude::Event::on_dragon_spawn) and [`Server::on_dragon_spawn`](crate::server::Server::on_dragon_spawn).
*/
pub struct DragonSpawnEvent {
    /// Runtime id the dragon will have.
    pub entity: u64,
    /// X coordinate, in blocks.
    pub x: f32,
    /// Y coordinate, in blocks.
    pub y: f32,
    /// Z coordinate, in blocks.
    pub z: f32,
    /// `true` when players respawn it with end crystals.
    pub respawn: bool,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
