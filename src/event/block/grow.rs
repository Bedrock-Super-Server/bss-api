use std::cell::Cell;

/**
A plant grows a stage: crops, stems, sweet berries, cocoa, cactus, sugar cane...

`event.cancel()` stops this growth step.

Handled by [`Event::on_block_grow`](crate::prelude::Event::on_block_grow) and [`Server::on_block_grow`](crate::server::Server::on_block_grow).
*/
pub struct BlockGrowEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block before it grows.
    pub block: u32,
    pub(crate) new_block: Cell<u32>,
    /// `true` when the growth comes from bone meal, `false` for natural growth.
    pub bone_meal: bool,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl BlockGrowEvent {
    /// Returns the runtime id of the block after it grows.
    pub fn new_block(&self) -> u32 {
        self.new_block.get()
    }

    /// Sets the runtime id of the block after it grows.
    pub fn set_new_block(&self, value: u32) {
        self.new_block.set(value);
    }
}
