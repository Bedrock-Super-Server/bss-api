/**
A container window is about to open for a player.

`event.cancel()` keeps the window closed.

Handled by [`Event::on_inventory_open`](crate::prelude::Event::on_inventory_open).
*/
pub struct InventoryOpenEvent {
    /// Name of the player.
    pub name: String,
    /// Id of the window.
    pub window_id: u8,
    /// Type of the container, as the Bedrock protocol numbers it.
    pub window_type: u8,
}

impl InventoryOpenEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
