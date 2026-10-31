/**
A player takes the result out of a smithing table.

`event.cancel()` cancels the operation.

Handled by [`Event::on_smithing_use`](crate::prelude::Event::on_smithing_use) and [`Server::on_smithing_use`](crate::server::Server::on_smithing_use).
*/
pub struct SmithingUseEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the base item.
    pub item: i16,
    /// Numeric id of the resulting item.
    pub result: i16,
}

impl SmithingUseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
