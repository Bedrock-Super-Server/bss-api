use std::cell::Cell;

/**
A block forms in the world on its own: lava and water make obsidian or cobblestone, snow piles up, ice freezes...

`event.cancel()` prevents the block from forming.

Handled by [`Event::on_block_form`](crate::prelude::Event::on_block_form) and [`Server::on_block_form`](crate::server::Server::on_block_form).
*/
pub struct BlockFormEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block being replaced.
    pub block: u32,
    pub(crate) new_block: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl BlockFormEvent {
    /// Returns the runtime id of the block that forms.
    pub fn new_block(&self) -> u32 {
        self.new_block.get()
    }

    /// Sets the runtime id of the block that forms.
    pub fn set_new_block(&self, value: u32) {
        self.new_block.set(value);
    }
}
