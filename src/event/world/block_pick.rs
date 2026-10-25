/**
A player picks a block with the middle mouse button.

Returning `true` from a handler cancels the pick.

Handled by [`Event::on_block_pick`](crate::prelude::Event::on_block_pick).
*/
pub struct BlockPickEvent {
    /// Name of the player.
    pub name: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block picked.
    pub runtime_id: u32,
    /// Numeric id of the item the player receives.
    pub item_id: i16,
    /// Hotbar slot the item goes to.
    pub hotbar_slot: u32,
}
