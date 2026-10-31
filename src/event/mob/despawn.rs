/**
A mob is about to despawn because no player is close enough.

`event.cancel()` keeps the mob.

Handled by [`Event::on_mob_despawn`](crate::prelude::Event::on_mob_despawn) and [`Server::on_mob_despawn`](crate::server::Server::on_mob_despawn).
*/
pub struct MobDespawnEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
