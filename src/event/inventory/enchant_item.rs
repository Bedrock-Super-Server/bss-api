use std::cell::Cell;

/**
A player enchants an item at an enchanting table.

Returning `true` from a handler cancels the enchantment.

Handled by [`Event::on_enchant_item`](crate::prelude::Event::on_enchant_item) and [`Server::on_enchant_item`](crate::server::Server::on_enchant_item).
*/
pub struct EnchantItemEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the enchanted item.
    pub item: i16,
    /// Which of the three offered enchantments was chosen, from `0` to `2`.
    pub option: i32,
    pub(crate) levels: Cell<i32>,
}

impl EnchantItemEvent {
    /// Returns the experience levels the enchantment costs.
    pub fn levels(&self) -> i32 {
        self.levels.get()
    }

    /// Sets the experience levels the enchantment costs.
    pub fn set_levels(&self, value: i32) {
        self.levels.set(value);
    }
}
