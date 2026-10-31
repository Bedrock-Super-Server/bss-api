/**
A player lies down in a bed: it is night or a thunderstorm, no monster is near and nobody else lies there.

`event.cancel()` keeps the player standing; the bed still becomes their respawn point.

Handled by [`Event::on_player_bed_enter`](crate::prelude::Event::on_player_bed_enter) and [`Server::on_player_bed_enter`](crate::server::Server::on_player_bed_enter).
*/
pub struct PlayerBedEnterEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the head of the bed, in blocks.
    pub x: i32,
    /// Y coordinate of the head of the bed, in blocks.
    pub y: i32,
    /// Z coordinate of the head of the bed, in blocks.
    pub z: i32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl PlayerBedEnterEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
