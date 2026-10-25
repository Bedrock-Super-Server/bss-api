/**
An item entity appears in the world: a drop, a broken block, a dispenser throw...

Returning `true` from a handler prevents the item from appearing.

Handled by [`Event::on_item_spawn`](crate::prelude::Event::on_item_spawn) and [`Server::on_item_spawn`](crate::server::Server::on_item_spawn).
*/
pub struct ItemSpawnEvent {
    /// Runtime id of the item entity.
    pub entity: u64,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Number of items.
    pub count: u32,
    /// X coordinate, in blocks.
    pub x: f32,
    /// Y coordinate, in blocks.
    pub y: f32,
    /// Z coordinate, in blocks.
    pub z: f32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
