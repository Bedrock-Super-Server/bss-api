/**
A player inserts a music disc into a jukebox or takes it out.

`event.cancel()` leaves the jukebox as it was.

Handled by [`Event::on_jukebox_change`](crate::prelude::Event::on_jukebox_change) and [`Server::on_jukebox_change`](crate::server::Server::on_jukebox_change).
*/
pub struct JukeboxChangeEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Numeric id of the disc.
    pub disc: i16,
    /// `true` when the disc goes in, `false` when it comes out.
    pub inserted: bool,
}

impl JukeboxChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
