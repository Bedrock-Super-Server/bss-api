/**
A furnace finishes smelting an item.

Handled by [`Event::on_furnace_smelt`](crate::prelude::Event::on_furnace_smelt).
*/
pub struct FurnaceSmeltEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Numeric id of the item smelted.
    pub input_item_id: i16,
    /// Numeric id of the result.
    pub result_item_id: i16,
}
