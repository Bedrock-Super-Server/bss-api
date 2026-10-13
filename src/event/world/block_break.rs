use std::cell::Cell;

/**
A player breaks a block.

Returning `true` from a handler cancels the break: the block is put back.

Handled by [`Event::on_block_break`](crate::prelude::Event::on_block_break).
*/
pub struct BlockBreakEvent {
    /// Name of the player.
    pub name: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block broken.
    pub runtime_id: u32,
    /// Numeric id of the item the block drops by default.
    pub item_id: i16,
    /// Numeric id of the item in the player's hand.
    pub held_item: i16,
    /// Unique id of the item in the player's hand.
    pub held_uid: u64,
    /// Damage of the item in the player's hand.
    pub held_damage: u16,
    /// `true` when the block is actually broken, `false` for an attempt the server is checking.
    pub final_break: bool,
    pub(crate) drop: Cell<bool>,
    pub(crate) drop_item: Cell<i16>,
    pub(crate) drop_count: Cell<u8>,
}

impl BlockBreakEvent {
    /// Returns whether the block drops an item.
    pub fn drop(&self) -> bool {
        self.drop.get()
    }

    /// Sets whether the block drops an item.
    pub fn set_drop(&self, value: bool) {
        self.drop.set(value);
    }

    /// Returns the numeric id of the item dropped.
    pub fn drop_item(&self) -> i16 {
        self.drop_item.get()
    }

    /// Sets the numeric id of the item dropped.
    pub fn set_drop_item(&self, value: i16) {
        self.drop_item.set(value);
    }

    /// Returns the number of items dropped.
    pub fn drop_count(&self) -> u8 {
        self.drop_count.get()
    }

    /// Sets the number of items dropped.
    pub fn set_drop_count(&self, value: u8) {
        self.drop_count.set(value);
    }
}
