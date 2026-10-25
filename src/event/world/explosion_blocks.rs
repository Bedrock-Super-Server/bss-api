use std::cell::Cell;

/**
An explosion is about to destroy blocks.

Returning `true` from a handler keeps every block in place.

Handled by [`Event::on_explosion_blocks`](crate::prelude::Event::on_explosion_blocks) and [`Server::on_explosion_blocks`](crate::server::Server::on_explosion_blocks).
*/
pub struct ExplosionBlocksEvent {
    /// X coordinate of the centre.
    pub x: f32,
    /// Y coordinate of the centre.
    pub y: f32,
    /// Z coordinate of the centre.
    pub z: f32,
    /// Number of blocks destroyed.
    pub count: u32,
    pub(crate) drop_items: Cell<bool>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl ExplosionBlocksEvent {
    /// Returns whether the destroyed blocks drop items.
    pub fn drop_items(&self) -> bool {
        self.drop_items.get()
    }

    /// Sets whether the destroyed blocks drop items.
    pub fn set_drop_items(&self, value: bool) {
        self.drop_items.set(value);
    }
}
