/**
A player is about to be teleported.

`event.cancel()` cancels the teleport.

Handled by [`Event::on_teleport`](crate::prelude::Event::on_teleport).
*/
pub struct TeleportEvent {
    /// Name of the player.
    pub name: String,
    /// Position before the teleport.
    pub from: (f32, f32, f32),
    /// Position after the teleport.
    pub to: (f32, f32, f32),
}

impl TeleportEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
