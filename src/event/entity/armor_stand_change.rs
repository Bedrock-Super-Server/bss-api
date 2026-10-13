/**
A player puts an item on an armor stand or takes one off.

Returning `true` from a handler leaves the armor stand as it was.

Handled by [`Event::on_armor_stand_change`](crate::prelude::Event::on_armor_stand_change) and [`Server::on_armor_stand_change`](crate::server::Server::on_armor_stand_change).
*/
pub struct ArmorStandChangeEvent {
    /// Name of the player.
    pub player: String,
    /// Runtime id of the armor stand.
    pub entity: u64,
    /// Equipment slot of the armor stand that changes.
    pub slot: u32,
    /// Numeric id of the item placed, or `0` when the slot is emptied.
    pub item: i16,
}
