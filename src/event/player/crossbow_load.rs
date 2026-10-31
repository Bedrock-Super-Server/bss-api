/**
A player loads a crossbow.

`event.cancel()` keeps the crossbow unloaded.

Handled by [`Event::on_crossbow_load`](crate::prelude::Event::on_crossbow_load) and [`Server::on_crossbow_load`](crate::server::Server::on_crossbow_load).
*/
pub struct CrossbowLoadEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the loaded ammunition: an arrow or a firework rocket.
    pub ammo: i16,
}

impl CrossbowLoadEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
