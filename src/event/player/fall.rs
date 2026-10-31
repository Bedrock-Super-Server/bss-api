use std::cell::Cell;

/**
A player lands after a fall that hurts them. Set another damage to hurt them less or more; cancelled, they take none.

`event.cancel()` saves the player from the fall: they take no damage.

Handled by [`Event::on_player_fall`](crate::prelude::Event::on_player_fall) and [`Server::on_player_fall`](crate::server::Server::on_player_fall).
*/
pub struct PlayerFallEvent {
    /// Name of the player.
    pub player: String,
    /// How far the player fell, in blocks.
    pub distance: f32,
    /// What the fall costs them, in half-hearts, after armor and enchantments; `0` saves them.
    pub(crate) damage: Cell<f32>,
    /// Name of the block they landed on, such as `minecraft:hay_block`.
    pub block: String,
}

impl PlayerFallEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl PlayerFallEvent {
    /// Returns the current value of `damage`, which the handlers before this one may have changed.
    ///
    /// What the fall costs them, in half-hearts, after armor and enchantments; `0` saves them.
    pub fn damage(&self) -> f32 {
        self.damage.get()
    }

    /// Sets `damage`: the server uses the last value set.
    ///
    /// What the fall costs them, in half-hearts, after armor and enchantments; `0` saves them.
    pub fn set_damage(&self, value: f32) {
        self.damage.set(value);
    }
}
