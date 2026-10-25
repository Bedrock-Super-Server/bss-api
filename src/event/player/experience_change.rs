use std::cell::Cell;

/**
A player gains or loses experience.

Returning `true` from a handler leaves the experience unchanged.

Handled by [`Event::on_experience_change`](crate::prelude::Event::on_experience_change) and [`Server::on_experience_change`](crate::server::Server::on_experience_change).
*/
pub struct ExperienceChangeEvent {
    /// Name of the player.
    pub player: String,
    /// Experience level before the change.
    pub level: i32,
    /// Total experience points before the change.
    pub total: i32,
    pub(crate) amount: Cell<i32>,
}

impl ExperienceChangeEvent {
    /// Returns the experience points added; negative to remove some.
    pub fn amount(&self) -> i32 {
        self.amount.get()
    }

    /// Sets the experience points added; negative to remove some.
    pub fn set_amount(&self, value: i32) {
        self.amount.set(value);
    }
}
