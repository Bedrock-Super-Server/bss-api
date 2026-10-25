/**
A player equips a piece of armor.

Handled by [`Event::on_armor_equip`](crate::prelude::Event::on_armor_equip).
*/
pub struct ArmorEquipEvent {
    /// Name of the player.
    pub name: String,
    /// Numeric id of the armor item.
    pub item_id: i16,
    /// Size of the stack.
    pub count: u8,
    /// Runtime id of the block form of the item, or `0` for an item that is not a block.
    pub block_runtime_id: u32,
    /// Inventory slot the armor comes from.
    pub source_slot: u32,
    /// Armor slot the piece goes to.
    pub armor_slot: u32,
}
