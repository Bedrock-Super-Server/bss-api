/**
A player moves to another world, or to another dimension of one.

`event.cancel()` changes nothing.

Handled by [`Event::on_player_world_change`](crate::prelude::Event::on_player_world_change) and [`Server::on_player_world_change`](crate::server::Server::on_player_world_change).
*/
pub struct PlayerWorldChangeEvent {
    /// Name of the player.
    pub player: String,
    /// Id of the world they leave, in load order: the default world is `0`.
    pub from_world: u32,
    /// Id of the world they arrive in.
    pub to_world: u32,
    /// Why: `portal`, `respawn` or `plugin` (a command or a call of the API).
    pub cause: String,
}

impl PlayerWorldChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
