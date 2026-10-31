/**
A player uses an item on a mob: feeds, tames, shears, milks, names...

`event.cancel()` cancels the interaction.

Handled by [`Event::on_mob_interact`](crate::prelude::Event::on_mob_interact) and [`Server::on_mob_interact`](crate::server::Server::on_mob_interact).
*/
pub struct MobInteractEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// Numeric id of the item in the player's hand, or `0` for an empty hand.
    pub item: i16,
    /// The interaction, such as `feed`, `tame`, `shear` or `milk`.
    pub action: String,
}

impl MobInteractEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
