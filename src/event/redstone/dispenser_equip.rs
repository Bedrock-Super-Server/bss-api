/**
A dispenser puts armor on a player in front of it.

`event.cancel()` keeps the armor in the dispenser.

Handled by [`Event::on_dispenser_equip`](crate::prelude::Event::on_dispenser_equip) and [`Server::on_dispenser_equip`](crate::server::Server::on_dispenser_equip).
*/
pub struct DispenserEquipEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Name of the player.
    pub player: String,
    /// Numeric id of the armor.
    pub item: i16,
    /// Armor slot the piece goes to.
    pub slot: u32,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl DispenserEquipEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
