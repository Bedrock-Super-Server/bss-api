/**
A world finishes loading and is ready for players and plugins.

`event.cancel()` changes nothing.

Handled by [`Event::on_world_load`](crate::prelude::Event::on_world_load) and [`Server::on_world_load`](crate::server::Server::on_world_load).
*/
pub struct WorldLoadEvent {
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
    /// Name of the world.
    pub name: String,
}
