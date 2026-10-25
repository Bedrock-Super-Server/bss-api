use std::cell::Cell;

/**
A player takes the result out of a grindstone.

Returning `true` from a handler cancels the operation.

Handled by [`Event::on_grindstone_use`](crate::prelude::Event::on_grindstone_use) and [`Server::on_grindstone_use`](crate::server::Server::on_grindstone_use).
*/
pub struct GrindstoneUseEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the resulting item.
    pub item: i16,
    pub(crate) experience: Cell<i32>,
}

impl GrindstoneUseEvent {
    /// Returns the experience the grindstone gives back.
    pub fn experience(&self) -> i32 {
        self.experience.get()
    }

    /// Sets the experience the grindstone gives back.
    pub fn set_experience(&self, value: i32) {
        self.experience.set(value);
    }
}
