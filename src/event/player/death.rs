use std::cell::{Cell, RefCell};

/**
A player dies.

Handled by [`Event::on_death`](crate::prelude::Event::on_death).
*/
pub struct DeathEvent {
    /// Name of the player.
    pub name: String,
    /// Name of the player or type of the entity that killed them, or an empty string.
    pub killer: String,
    pub(crate) message: RefCell<String>,
    pub(crate) keep_inventory: Cell<bool>,
}

impl DeathEvent {
    /// Returns the death message shown in chat.
    pub fn message(&self) -> String {
        self.message.borrow().clone()
    }

    /// Sets the death message shown in chat.
    pub fn set_message(&self, value: impl Into<String>) {
        *self.message.borrow_mut() = value.into();
    }

    /// Returns whether the player keeps their inventory and experience: the `keepInventory` game rule, unless a plugin changed it.
    pub fn keep_inventory(&self) -> bool {
        self.keep_inventory.get()
    }

    /// Sets whether the player keeps their inventory and experience.
    pub fn set_keep_inventory(&self, value: bool) {
        self.keep_inventory.set(value);
    }
}

impl DeathEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
