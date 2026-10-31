/**
A sapling or a fungus grows into a tree.

`event.cancel()` keeps the sapling as it is.

Handled by [`Event::on_tree_grow`](crate::prelude::Event::on_tree_grow) and [`Server::on_tree_grow`](crate::server::Server::on_tree_grow).
*/
pub struct TreeGrowEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the sapling block.
    pub sapling: u32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}
