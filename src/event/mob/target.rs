use std::cell::Cell;

/**
A mob picks a new target.

Returning `true` from a handler leaves the mob without a target.

Handled by [`Event::on_mob_target`](crate::prelude::Event::on_mob_target) and [`Server::on_mob_target`](crate::server::Server::on_mob_target).
*/
pub struct MobTargetEvent {
    /// Runtime id of the mob.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    pub(crate) target: Cell<u64>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl MobTargetEvent {
    /// Returns the runtime id of the target, or `0` for none.
    pub fn target(&self) -> u64 {
        self.target.get()
    }

    /// Sets the runtime id of the target, or `0` for none.
    pub fn set_target(&self, value: u64) {
        self.target.set(value);
    }
}
