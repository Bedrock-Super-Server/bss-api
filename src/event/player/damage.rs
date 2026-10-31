use std::cell::Cell;

/**
A player takes damage.

`event.cancel()` cancels the damage.

Handled by [`Event::on_player_damage`](crate::prelude::Event::on_player_damage) and [`Server::on_player_damage`](crate::server::Server::on_player_damage).
*/
pub struct PlayerDamageEvent {
    /// Name of the player.
    pub player: String,
    /// What caused the damage, such as `attack`, `projectile`, `fall`, `fire`, `lava`, `drowning`, `explosion` or `void`.
    pub source: String,
    pub(crate) damage: Cell<f32>,
    /// Name of the player or type of the entity responsible, or an empty string.
    pub attacker: String,
}

impl PlayerDamageEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl PlayerDamageEvent {
    /// Returns the damage taken, in half-hearts.
    pub fn damage(&self) -> f32 {
        self.damage.get()
    }

    /// Sets the damage taken, in half-hearts.
    pub fn set_damage(&self, value: f32) {
        self.damage.set(value);
    }
}
