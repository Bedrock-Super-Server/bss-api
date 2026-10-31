/**
A tool, weapon or piece of armor breaks after losing its last durability point.

Handled by [`Event::on_item_break`](crate::prelude::Event::on_item_break) and [`Server::on_item_break`](crate::server::Server::on_item_break).
*/
pub struct ItemBreakEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Inventory slot of the item; armor pieces use `100` plus their armor slot.
    pub slot: u32,
}

impl ItemBreakEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
