/**
A player moves or turns.

`event.cancel()` moves the player back to where they were.

Handled by [`Event::on_move`](crate::prelude::Event::on_move).
*/
pub struct MoveEvent {
    /// Name of the player.
    pub name: String,
    /// Position before the move.
    pub from: (f32, f32, f32),
    /// Position after the move.
    pub to: (f32, f32, f32),
    /// Pitch of the head, in degrees.
    pub pitch: f32,
    /// Yaw of the head, in degrees.
    pub yaw: f32,
    /// `true` while the player sneaks.
    pub sneaking: bool,
    /// `true` while the player sprints.
    pub sprinting: bool,
}

impl MoveEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
