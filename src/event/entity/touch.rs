/**
A player touches or hits an entity that has no behaviour of its own, such as a plugin entity.

Returning `true` from a handler ignores the touch.

Handled by [`Event::on_entity_touch`](crate::prelude::Event::on_entity_touch) and [`Server::on_entity_touch`](crate::server::Server::on_entity_touch).
*/
pub struct EntityTouchEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// `true` for a hit, `false` for a use.
    pub attack: bool,
}
