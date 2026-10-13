/**
A player starts, continues or stops breaking a block.

Returning `true` from a handler stops the progress.

Handled by [`Event::on_block_break_progress`](crate::prelude::Event::on_block_break_progress).
*/
pub struct BlockBreakProgressEvent {
    /// Name of the player.
    pub name: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the block.
    pub runtime_id: u32,
    /// Stage reported; [`is_start`](Self::is_start) and [`is_abort`](Self::is_abort) tell them apart.
    pub stage: u32,
}

impl BlockBreakProgressEvent {
    /// Returns `true` when the player starts breaking the block.
    pub fn is_start(&self) -> bool {
        self.stage == 0
    }

    /// Returns `true` when the player stops before the block breaks.
    pub fn is_abort(&self) -> bool {
        self.stage == 1
    }
}
