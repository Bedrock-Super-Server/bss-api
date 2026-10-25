use std::cell::Cell;

/**
A player blows a goat horn.

Returning `true` from a handler keeps the horn silent.

Handled by [`Event::on_horn_blow`](crate::prelude::Event::on_horn_blow) and [`Server::on_horn_blow`](crate::server::Server::on_horn_blow).
*/
pub struct HornBlowEvent {
    /// Name of the player.
    pub player: String,
    pub(crate) variant: Cell<i32>,
}

impl HornBlowEvent {
    /// Returns the horn variant, which sets the sound played.
    pub fn variant(&self) -> i32 {
        self.variant.get()
    }

    /// Sets the horn variant, which sets the sound played.
    pub fn set_variant(&self, value: i32) {
        self.variant.set(value);
    }
}
