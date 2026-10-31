use std::cell::RefCell;

/**
A line is typed in the server console, before it runs. Change `command` to run something else.

`event.cancel()` keeps the line from running.

Handled by [`Event::on_console_command`](crate::prelude::Event::on_console_command) and [`Server::on_console_command`](crate::server::Server::on_console_command).
*/
pub struct ConsoleCommandEvent {
    /// The line typed, without the leading slash.
    pub(crate) command: RefCell<String>,
}

impl ConsoleCommandEvent {
    /// Returns the current value of `command`, which the handlers before this one may have changed.
    ///
    /// The line typed, without the leading slash.
    pub fn command(&self) -> String {
        self.command.borrow().clone()
    }

    /// Sets `command`: the server uses the last value set.
    ///
    /// The line typed, without the leading slash.
    pub fn set_command(&self, value: impl Into<String>) {
        *self.command.borrow_mut() = value.into();
    }
}
