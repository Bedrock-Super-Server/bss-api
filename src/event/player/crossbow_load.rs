/**
A player loads a crossbow.

Returning `true` from a handler keeps the crossbow unloaded.

Handled by [`Event::on_crossbow_load`](crate::prelude::Event::on_crossbow_load) and [`Server::on_crossbow_load`](crate::server::Server::on_crossbow_load).
*/
pub struct CrossbowLoadEvent {
    /// Name of the player.
    pub player: String,
    /// Numeric id of the loaded ammunition: an arrow or a firework rocket.
    pub ammo: i16,
}
