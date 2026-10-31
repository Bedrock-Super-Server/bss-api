/**
A player blocks an attack with a shield.

`event.cancel()` cancels it.

Handled by [`Event::on_shield_block`](crate::prelude::Event::on_shield_block) and [`Server::on_shield_block`](crate::server::Server::on_shield_block).
*/
pub struct ShieldBlockEvent {
    /// Name of the player.
    pub player: String,
    /// Damage the shield blocked, in half-hearts.
    pub damage: f32,
    /// Name of the player or type of the entity that attacked.
    pub attacker: String,
}

impl ShieldBlockEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
