/**
A player attacks an entity or another player.

`event.cancel()` stops the hit: nothing is hurt and the weapon does not wear.

Handled by [`Event::on_player_attack`](crate::prelude::Event::on_player_attack) and [`Server::on_player_attack`](crate::server::Server::on_player_attack).
*/
pub struct PlayerAttackEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of what is attacked.
    pub entity: u64,
    /// Name of the player attacked, empty when it is an entity.
    pub victim: String,
    /// Numeric id of the item in the attacker's hand, `0` for an empty hand.
    pub item: i16,
}

impl PlayerAttackEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
