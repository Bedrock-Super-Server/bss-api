use std::cell::Cell;

/**
A tool, weapon or piece of armor is about to lose durability.

Returning `true` from a handler prevents the loss.

Handled by [`Event::on_item_damage`](crate::prelude::Event::on_item_damage) and [`Server::on_item_damage`](crate::server::Server::on_item_damage).
*/
pub struct ItemDamageEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Inventory slot of the item; armor pieces use `100` plus their armor slot.
    pub slot: u32,
    pub(crate) damage: Cell<u32>,
}

impl ItemDamageEvent {
    /// Returns the durability points lost.
    pub fn damage(&self) -> u32 {
        self.damage.get()
    }

    /// Sets the durability points lost.
    pub fn set_damage(&self, value: u32) {
        self.damage.set(value);
    }
}
