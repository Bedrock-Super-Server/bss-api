/**
A player uses the item in their hand, on a block or in the air.

`event.cancel()` cancels the use.

Handled by [`Event::on_item_use`](crate::prelude::Event::on_item_use).
*/
pub struct ItemUseEvent {
    /// Name of the player.
    pub name: String,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item_id: i16,
    /// Runtime id of the block form of the item, or `0` for an item that is not a block.
    pub block_runtime_id: u32,
    /// [`CLICK_BLOCK`](Self::CLICK_BLOCK) or [`CLICK_AIR`](Self::CLICK_AIR).
    pub action: u32,
    /// X coordinate of the block, for a use on a block.
    pub x: i32,
    /// Y coordinate of the block, for a use on a block.
    pub y: i32,
    /// Z coordinate of the block, for a use on a block.
    pub z: i32,
    /// Face of the block the player clicked, from `0` to `5`.
    pub face: u8,
    /// Hotbar slot the player had selected, from `0` to `8`.
    pub hotbar_slot: i32,
    /// Where the player clicked on the block face, along X, from `0.0` to `1.0`.
    pub click_x: f32,
    /// Where the player clicked on the block face, along Y, from `0.0` to `1.0`.
    pub click_y: f32,
    /// Where the player clicked on the block face, along Z, from `0.0` to `1.0`.
    pub click_z: f32,
}

impl ItemUseEvent {
    /// [`action`](Self::action) of a use on a block.
    pub const CLICK_BLOCK: u32 = 0;
    /// [`action`](Self::action) of a use in the air.
    pub const CLICK_AIR: u32 = 1;

    /// Returns `true` when the item is used on a block.
    pub fn is_block(&self) -> bool {
        self.action == Self::CLICK_BLOCK
    }

    /// Returns `true` when the item is used in the air.
    pub fn is_air(&self) -> bool {
        self.action == Self::CLICK_AIR
    }
}

impl ItemUseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
