/**
Something asks to disconnect a player: a plugin, the `/kick` command or the server's own protections.

Returning `true` from a handler keeps the player connected.

Handled by [`Event::on_player_kick`](crate::prelude::Event::on_player_kick) and [`Server::on_player_kick`](crate::server::Server::on_player_kick).
*/
pub struct PlayerKickEvent {
    /// Name of the player.
    pub player: String,
    /// Why the player is disconnected, as written in the log.
    pub reason: String,
}
