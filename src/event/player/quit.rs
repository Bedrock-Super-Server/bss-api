use std::cell::RefCell;

/**
A player leaves the server.

Handled by [`Event::on_quit`](crate::prelude::Event::on_quit).
*/
pub struct QuitEvent {
    /// Name of the player.
    pub name: String,
    pub(crate) message: RefCell<String>,
}

impl QuitEvent {
    /// Returns the quit message shown in chat; empty to show none.
    pub fn message(&self) -> String {
        self.message.borrow().clone()
    }

    /// Sets the quit message shown in chat; empty to show none.
    pub fn set_message(&self, message: &str) {
        *self.message.borrow_mut() = message.to_string();
    }
}
