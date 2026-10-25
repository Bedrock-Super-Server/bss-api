/**
A player opens the server settings page of their game settings.

Handled by [`Event::on_server_settings_request`](crate::prelude::Event::on_server_settings_request).
*/
pub struct ServerSettingsRequestEvent {
    /// Name of the player.
    pub name: String,
}
