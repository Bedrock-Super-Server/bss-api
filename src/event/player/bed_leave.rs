/**
A player gets out of bed. `cause` is `morning` when the world woke everyone up, `manual` when the player left by themselves, `damage`, `gamemode` or `travel` when something forced them out, and `bed_destroyed` when the bed is gone.

`event.cancel()` changes nothing: the player leaves the bed anyway.

Handled by [`Event::on_player_bed_leave`](crate::prelude::Event::on_player_bed_leave) and [`Server::on_player_bed_leave`](crate::server::Server::on_player_bed_leave).
*/
pub struct PlayerBedLeaveEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the head of the bed, in blocks.
    pub x: i32,
    /// Y coordinate of the head of the bed, in blocks.
    pub y: i32,
    /// Z coordinate of the head of the bed, in blocks.
    pub z: i32,
    /// Why the player got up: `morning`, `manual`, `damage`, `gamemode`, `travel` or `bed_destroyed`.
    pub cause: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl PlayerBedLeaveEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
