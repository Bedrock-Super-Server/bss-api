use std::cell::Cell;

/**
A sculk shrieker shrieks because a player stepped near it or made a vibration it heard.

Returning `true` from a handler keeps the shrieker silent.

Handled by [`Event::on_sculk_shriek`](crate::prelude::Event::on_sculk_shriek) and [`Server::on_sculk_shriek`](crate::server::Server::on_sculk_shriek).
*/
pub struct SculkShriekEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Name of the player who caused the shriek.
    pub player: String,
    pub(crate) warning: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl SculkShriekEvent {
    /// Returns the warning level after this shriek; at level 4 a warden spawns.
    pub fn warning(&self) -> u32 {
        self.warning.get()
    }

    /// Sets the warning level after this shriek; at level 4 a warden spawns.
    pub fn set_warning(&self, value: u32) {
        self.warning.set(value);
    }
}
