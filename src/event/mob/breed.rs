use std::cell::Cell;

/**
Two mobs breed and a baby is about to be born.

Returning `true` from a handler prevents the birth.

Handled by [`Event::on_mob_breed`](crate::prelude::Event::on_mob_breed) and [`Server::on_mob_breed`](crate::server::Server::on_mob_breed).
*/
pub struct MobBreedEvent {
    /// Runtime id of the first parent.
    pub parent_a: u64,
    /// Runtime id of the second parent.
    pub parent_b: u64,
    /// Type of the baby, such as `minecraft:cow`.
    pub identifier: String,
    pub(crate) experience: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl MobBreedEvent {
    /// Returns the experience dropped for the player who fed them.
    pub fn experience(&self) -> u32 {
        self.experience.get()
    }

    /// Sets the experience dropped for the player who fed them.
    pub fn set_experience(&self, value: u32) {
        self.experience.set(value);
    }
}
