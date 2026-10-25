use std::cell::Cell;

/**
TNT is primed and starts its fuse.

Returning `true` from a handler keeps the TNT block unprimed.

Handled by [`Event::on_tnt_prime`](crate::prelude::Event::on_tnt_prime) and [`Server::on_tnt_prime`](crate::server::Server::on_tnt_prime).
*/
pub struct TntPrimeEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) fuse: Cell<u32>,
    /// What primed it, such as `redstone`, `explosion`, `arrow`, `dispenser` or `activator_rail`.
    pub cause: String,
    /// Runtime id of the entity responsible, such as the player who shot the arrow, or `0`.
    pub owner: u64,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl TntPrimeEvent {
    /// Returns how long the fuse burns before the explosion, in ticks.
    pub fn fuse(&self) -> u32 {
        self.fuse.get()
    }

    /// Sets how long the fuse burns before the explosion, in ticks.
    pub fn set_fuse(&self, value: u32) {
        self.fuse.set(value);
    }
}
