/**
A dropped item disappears after lying on the ground for five minutes.

Handled by [`Event::on_item_despawn`](crate::prelude::Event::on_item_despawn) and [`Server::on_item_despawn`](crate::server::Server::on_item_despawn).
*/
pub struct ItemDespawnEvent {
    /// Runtime id of the item entity.
    pub entity: u64,
    /// Numeric id of the item; [`Server::item_name`](crate::server::Server::item_name) gives its name.
    pub item: i16,
    /// Number of items.
    pub count: u32,
}
