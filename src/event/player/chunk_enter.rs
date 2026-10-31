/**
A player moves into another chunk.

`event.cancel()` changes nothing.

Handled by [`Event::on_player_chunk_enter`](crate::prelude::Event::on_player_chunk_enter) and [`Server::on_player_chunk_enter`](crate::server::Server::on_player_chunk_enter).
*/
pub struct PlayerChunkEnterEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the chunk they enter, in chunks.
    pub chunk_x: i32,
    /// Z coordinate of the chunk they enter, in chunks.
    pub chunk_z: i32,
    /// X coordinate of the chunk they leave, in chunks.
    pub from_x: i32,
    /// Z coordinate of the chunk they leave, in chunks.
    pub from_z: i32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl PlayerChunkEnterEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
