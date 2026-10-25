/**
A player selects another hotbar slot.

Returning `true` from a handler keeps the previous slot selected.

Handled by [`Event::on_item_held`](crate::prelude::Event::on_item_held).
*/
pub struct ItemHeldEvent {
    /// Name of the player.
    pub name: String,
    /// Slot selected before, from `0` to `8`.
    pub old_slot: u32,
    /// Slot selected now, from `0` to `8`.
    pub new_slot: u32,
    /// Numeric id of the item in the new slot.
    pub item_id: i16,
}
