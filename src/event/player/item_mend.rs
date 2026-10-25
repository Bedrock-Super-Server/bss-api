use std::cell::Cell;

/**
Mending repairs an item with experience the player picked up.

Returning `true` from a handler cancels the repair.

Handled by [`Event::on_item_mend`](crate::prelude::Event::on_item_mend) and [`Server::on_item_mend`](crate::server::Server::on_item_mend).
*/
pub struct ItemMendEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    pub(crate) repair: Cell<u32>,
}

impl ItemMendEvent {
    /// Returns the durability points restored.
    pub fn repair(&self) -> u32 {
        self.repair.get()
    }

    /// Sets the durability points restored.
    pub fn set_repair(&self, value: u32) {
        self.repair.set(value);
    }
}
