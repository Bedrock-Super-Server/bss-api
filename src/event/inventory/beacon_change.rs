/**
A player changes the effects of a beacon.

`event.cancel()` keeps the previous effects.

Handled by [`Event::on_beacon_change`](crate::prelude::Event::on_beacon_change) and [`Server::on_beacon_change`](crate::server::Server::on_beacon_change).
*/
pub struct BeaconChangeEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Id of the primary effect.
    pub primary: i32,
    /// Id of the secondary effect, or `0` when there is none.
    pub secondary: i32,
}

impl BeaconChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
