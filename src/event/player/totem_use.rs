/**
A totem of undying saves a player from death.

Returning `true` from a handler stops the totem from saving the player.

Handled by [`Event::on_totem_use`](crate::prelude::Event::on_totem_use) and [`Server::on_totem_use`](crate::server::Server::on_totem_use).
*/
pub struct TotemUseEvent {
    /// Name of the player.
    pub player: String,
}
