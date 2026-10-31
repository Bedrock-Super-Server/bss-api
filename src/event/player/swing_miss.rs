/**
A player swings their arm and hits nothing: no entity, and no block they were breaking, using or placing.

`event.cancel()` changes nothing.

Handled by [`Event::on_player_swing_miss`](crate::prelude::Event::on_player_swing_miss) and [`Server::on_player_swing_miss`](crate::server::Server::on_player_swing_miss).
*/
pub struct PlayerSwingMissEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the player, in blocks.
    pub x: f32,
    /// Y coordinate of the player, in blocks.
    pub y: f32,
    /// Z coordinate of the player, in blocks.
    pub z: f32,
    /// Where the player looks up or down, in degrees.
    pub pitch: f32,
    /// Where the player looks around, in degrees.
    pub yaw: f32,
}

impl PlayerSwingMissEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
