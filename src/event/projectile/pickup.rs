/**
A player picks up an arrow or a trident stuck in the world.

Returning `true` from a handler leaves it in place.

Handled by [`Event::on_projectile_pickup`](crate::prelude::Event::on_projectile_pickup) and [`Server::on_projectile_pickup`](crate::server::Server::on_projectile_pickup).
*/
pub struct ProjectilePickupEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the projectile.
    pub entity: u64,
    /// Numeric id of the item the player receives.
    pub item: i16,
}
