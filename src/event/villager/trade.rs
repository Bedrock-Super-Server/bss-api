use std::cell::Cell;

/**
A player trades with a villager or a wandering trader.

`event.cancel()` cancels the trade.

Handled by [`Event::on_villager_trade`](crate::prelude::Event::on_villager_trade) and [`Server::on_villager_trade`](crate::server::Server::on_villager_trade).
*/
pub struct VillagerTradeEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the trader.
    pub entity: u64,
    /// Index of the offer in the trade list.
    pub offer: u32,
    /// How many times the offer is used in this trade.
    pub times: u32,
    /// Numeric id of the item the player receives.
    pub item: i16,
    pub(crate) experience: Cell<i32>,
}

impl VillagerTradeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl VillagerTradeEvent {
    /// Returns the experience the player receives.
    pub fn experience(&self) -> i32 {
        self.experience.get()
    }

    /// Sets the experience the player receives.
    pub fn set_experience(&self, value: i32) {
        self.experience.set(value);
    }
}
