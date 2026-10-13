/**
A player sends a command, before the server runs it.

Returning `true` from a handler cancels the command.

Handled by [`Event::on_command`](crate::prelude::Event::on_command).
*/
pub struct CommandEvent {
    /// Name of the player.
    pub name: String,
    /// The command line, without its leading `/`.
    pub command: String,
}
