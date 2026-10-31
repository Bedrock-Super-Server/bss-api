use std::cell::Cell;

/**
A dispenser or dropper fires an item.

`event.cancel()` cancels the shot.

Handled by [`Event::on_block_dispense`](crate::prelude::Event::on_block_dispense) and [`Server::on_block_dispense`](crate::server::Server::on_block_dispense).
*/
pub struct BlockDispenseEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) item: Cell<i16>,
    /// `true` for a dropper, `false` for a dispenser.
    pub dropper: bool,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl BlockDispenseEvent {
    /// Returns the numeric id of the item fired.
    pub fn item(&self) -> i16 {
        self.item.get()
    }

    /// Sets the numeric id of the item fired.
    pub fn set_item(&self, value: i16) {
        self.item.set(value);
    }
}
