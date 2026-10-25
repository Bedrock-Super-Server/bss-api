/**
A player jumps: their client presses jump while they stand on the ground.

Returning `true` changes nothing.

Handled by [`Event::on_player_jump`](crate::prelude::Event::on_player_jump) and [`Server::on_player_jump`](crate::server::Server::on_player_jump).
*/
pub struct PlayerJumpEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the player, in blocks.
    pub x: f32,
    /// Y coordinate of the player, in blocks.
    pub y: f32,
    /// Z coordinate of the player, in blocks.
    pub z: f32,
}
