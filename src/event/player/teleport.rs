/**
A player is about to be teleported.

Returning `true` from a handler cancels the teleport.

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
