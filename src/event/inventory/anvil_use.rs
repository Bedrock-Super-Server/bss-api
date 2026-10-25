use std::cell::Cell;

/**
A player takes the result out of an anvil.

Returning `true` from a handler cancels the operation.

Handled by [`Event::on_anvil_use`](crate::prelude::Event::on_anvil_use) and [`Server::on_anvil_use`](crate::server::Server::on_anvil_use).
*/
pub struct AnvilUseEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the resulting item.
    pub item: i16,
    /// New name given to the item, or an empty string when it is not renamed.
    pub name: String,
    pub(crate) levels: Cell<i32>,
}

impl AnvilUseEvent {
    /// Returns the experience levels the operation costs.
    pub fn levels(&self) -> i32 {
        self.levels.get()
    }

    /// Sets the experience levels the operation costs.
    pub fn set_levels(&self, value: i32) {
        self.levels.set(value);
    }
}
