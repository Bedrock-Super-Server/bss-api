/**
A player opens the server settings page of their game settings.

Handled by [`Event::on_server_settings_request`](crate::prelude::Event::on_server_settings_request).
*/
pub struct ServerSettingsRequestEvent {
    /// Name of the player.
    pub name: String,
}

impl ServerSettingsRequestEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
