/**
A totem of undying saves a player from death.

`event.cancel()` stops the totem from saving the player.

Handled by [`Event::on_totem_use`](crate::prelude::Event::on_totem_use) and [`Server::on_totem_use`](crate::server::Server::on_totem_use).
*/
pub struct TotemUseEvent {
    /// Name of the player.
    pub player: String,
}

impl TotemUseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
