use std::cell::Cell;

/**
A furnace, smoker, blast furnace or campfire finishes cooking an item.

Returning `true` from a handler cancels the result: the input stays in place.

Handled by [`Event::on_item_cook`](crate::prelude::Event::on_item_cook) and [`Server::on_item_cook`](crate::server::Server::on_item_cook).
*/
pub struct ItemCookEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Kind of station, such as `furnace` or `campfire`.
    pub station: String,
    /// Numeric id of the item that was cooked.
    pub input: i16,
    pub(crate) output: Cell<i16>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl ItemCookEvent {
    /// Returns the numeric id of the item the cooking produces.
    pub fn output(&self) -> i16 {
        self.output.get()
    }

    /// Sets the numeric id of the item the cooking produces.
    pub fn set_output(&self, value: i16) {
        self.output.set(value);
    }
}
