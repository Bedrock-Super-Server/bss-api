/**
A container window closes.

Handled by [`Event::on_inventory_close`](crate::prelude::Event::on_inventory_close).
*/
pub struct InventoryCloseEvent {
    /// Name of the player.
    pub name: String,
    /// Id of the window.
    pub window_id: u8,
    /// Type of the container, as the Bedrock protocol numbers it.
    pub window_type: u8,
    /// `true` when the server closed the window, `false` when the player did.
    pub server: bool,
}

impl InventoryCloseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
