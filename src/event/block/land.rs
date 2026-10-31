use std::cell::Cell;

/**
A falling block lands and is about to become a block again.

`event.cancel()` drops the block as an item instead of placing it.

Handled by [`Event::on_block_land`](crate::prelude::Event::on_block_land) and [`Server::on_block_land`](crate::server::Server::on_block_land).
*/
pub struct BlockLandEvent {
    /// Runtime id of the falling block entity.
    pub entity: u64,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) block: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl BlockLandEvent {
    /// Returns the runtime id of the block placed where it lands.
    pub fn block(&self) -> u32 {
        self.block.get()
    }

    /// Sets the runtime id of the block placed where it lands.
    pub fn set_block(&self, value: u32) {
        self.block.set(value);
    }
}
