/**
A player has done nothing for two minutes, or acts again.

`event.cancel()` changes nothing.

Handled by [`Event::on_player_afk`](crate::prelude::Event::on_player_afk) and [`Server::on_player_afk`](crate::server::Server::on_player_afk).
*/
pub struct PlayerAfkEvent {
    /// Name of the player.
    pub player: String,
    /// Whether the player is away now.
    pub afk: bool,
    /// How many seconds they did nothing.
    pub idle: u32,
}

impl PlayerAfkEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
