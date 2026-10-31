/**
A player sends a command, before the server runs it.

`event.cancel()` cancels the command.

Handled by [`Event::on_command`](crate::prelude::Event::on_command).
*/
pub struct CommandEvent {
    /// Name of the player.
    pub name: String,
    /// The command line, without its leading `/`.
    pub command: String,
}

impl CommandEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
