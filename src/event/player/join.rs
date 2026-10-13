use std::cell::RefCell;

/**
A player has finished joining and spawns in the world.

Handled by [`Event::on_join`](crate::prelude::Event::on_join).
*/
pub struct JoinEvent {
    /// Name of the player.
    pub name: String,
    pub(crate) message: RefCell<String>,
}

impl JoinEvent {
    /// Returns the join message shown in chat; empty to show none.
    pub fn message(&self) -> String {
        self.message.borrow().clone()
    }

    /// Sets the join message shown in chat; empty to show none.
    pub fn set_message(&self, message: &str) {
        *self.message.borrow_mut() = message.to_string();
    }
}
