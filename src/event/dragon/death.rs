use std::cell::Cell;

/**
The ender dragon dies.

Handled by [`Event::on_dragon_death`](crate::prelude::Event::on_dragon_death) and [`Server::on_dragon_death`](crate::server::Server::on_dragon_death).
*/
pub struct DragonDeathEvent {
    /// Runtime id of the dragon.
    pub entity: u64,
    /// Name of the player who dealt the last hit, or an empty string.
    pub killer: String,
    /// `true` for the first dragon killed in this world.
    pub first: bool,
    pub(crate) experience: Cell<u32>,
    pub(crate) egg: Cell<bool>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl DragonDeathEvent {
    /// Returns the experience the dragon drops.
    pub fn experience(&self) -> u32 {
        self.experience.get()
    }

    /// Sets the experience the dragon drops.
    pub fn set_experience(&self, value: u32) {
        self.experience.set(value);
    }

    /// Returns whether a dragon egg appears on the exit portal.
    pub fn egg(&self) -> bool {
        self.egg.get()
    }

    /// Sets whether a dragon egg appears on the exit portal.
    pub fn set_egg(&self, value: bool) {
        self.egg.set(value);
    }
}
