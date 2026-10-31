/**
A player changes the item in their off hand.

`event.cancel()` cancels it.

Handled by [`Event::on_offhand_change`](crate::prelude::Event::on_offhand_change) and [`Server::on_offhand_change`](crate::server::Server::on_offhand_change).
*/
pub struct OffhandChangeEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the new item, or `0` when the off hand is emptied.
    pub item: i16,
    /// Size of the new stack.
    pub count: u32,
}

impl OffhandChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
