use std::cell::Cell;

/**
A player picks up an experience orb.

`event.cancel()` leaves the orb where it is.

Handled by [`Event::on_experience_pickup`](crate::prelude::Event::on_experience_pickup) and [`Server::on_experience_pickup`](crate::server::Server::on_experience_pickup).
*/
pub struct ExperiencePickupEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the orb.
    pub entity: u64,
    pub(crate) amount: Cell<u32>,
}

impl ExperiencePickupEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl ExperiencePickupEvent {
    /// Returns the experience points the orb gives.
    pub fn amount(&self) -> u32 {
        self.amount.get()
    }

    /// Sets the experience points the orb gives.
    pub fn set_amount(&self, value: u32) {
        self.amount.set(value);
    }
}
