/**
A player right-clicks a block or the air.

`event.cancel()` cancels the interaction.

Handled by [`Event::on_interact`](crate::prelude::Event::on_interact).
*/
pub struct InteractEvent {
    /// Name of the player.
    pub name: String,
    /// [`RIGHT_CLICK_BLOCK`](Self::RIGHT_CLICK_BLOCK) or [`RIGHT_CLICK_AIR`](Self::RIGHT_CLICK_AIR).
    pub action: u32,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Face of the block the player clicked, from `0` to `5`.
    pub face: u32,
    /// Numeric id of the item in the player's hand.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0` for an item that is not a block.
    pub block_runtime_id: u32,
    /// Hotbar slot the player had selected, from `0` to `8`.
    pub hotbar_slot: i32,
    /// Where the player clicked on the block face, along X, from `0.0` to `1.0`.
    pub click_x: f32,
    /// Where the player clicked on the block face, along Y, from `0.0` to `1.0`.
    pub click_y: f32,
    /// Where the player clicked on the block face, along Z, from `0.0` to `1.0`.
    pub click_z: f32,
}

impl InteractEvent {
    /// [`action`](Self::action) of a click on a block.
    pub const RIGHT_CLICK_BLOCK: u32 = 0;
    /// [`action`](Self::action) of a click in the air.
    pub const RIGHT_CLICK_AIR: u32 = 1;
}

impl InteractEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
