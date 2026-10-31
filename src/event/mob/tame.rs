/**
A player tames a mob.

`event.cancel()` makes this taming attempt fail.

Handled by [`Event::on_mob_tame`](crate::prelude::Event::on_mob_tame) and [`Server::on_mob_tame`](crate::server::Server::on_mob_tame).
*/
pub struct MobTameEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl MobTameEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
