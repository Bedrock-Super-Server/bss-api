/**
A player places a block.

`event.cancel()` cancels the placement and keeps the item.

Handled by [`Event::on_block_place`](crate::prelude::Event::on_block_place).
*/
pub struct BlockPlaceEvent {
    /// Name of the player.
    pub name: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block placed.
    pub runtime_id: u32,
    /// Face of the block the player clicked.
    pub face: crate::math::BlockFace,
    /// Numeric id of the item used.
    pub item_id: i16,
}

impl BlockPlaceEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
