/**
A player places an entity from an item: an armor stand, a boat, a minecart, an end crystal...

`event.cancel()` cancels the placement and keeps the item.

Handled by [`Event::on_entity_place`](crate::prelude::Event::on_entity_place) and [`Server::on_entity_place`](crate::server::Server::on_entity_place).
*/
pub struct EntityPlaceEvent {
    /// Name of the player.
    pub player: String,
    /// Type of the entity placed, such as `minecraft:armor_stand`.
    pub identifier: String,
    /// X coordinate, in blocks.
    pub x: f32,
    /// Y coordinate, in blocks.
    pub y: f32,
    /// Z coordinate, in blocks.
    pub z: f32,
}

impl EntityPlaceEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
