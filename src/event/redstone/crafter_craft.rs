use std::cell::Cell;

/**
A crafter crafts an item.

Returning `true` from a handler cancels the craft.

Handled by [`Event::on_crafter_craft`](crate::prelude::Event::on_crafter_craft) and [`Server::on_crafter_craft`](crate::server::Server::on_crafter_craft).
*/
pub struct CrafterCraftEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Id of the recipe used.
    pub recipe: u32,
    pub(crate) item: Cell<i16>,
    pub(crate) count: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl CrafterCraftEvent {
    /// Returns the numeric id of the item crafted.
    pub fn item(&self) -> i16 {
        self.item.get()
    }

    /// Sets the numeric id of the item crafted.
    pub fn set_item(&self, value: i16) {
        self.item.set(value);
    }

    /// Returns the number of items crafted.
    pub fn count(&self) -> u32 {
        self.count.get()
    }

    /// Sets the number of items crafted.
    pub fn set_count(&self, value: u32) {
        self.count.set(value);
    }
}
