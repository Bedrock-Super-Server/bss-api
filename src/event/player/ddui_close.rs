/**
A data-driven screen closed.

`event.cancel()` changes nothing.

Handled by [`Event::on_ddui_close`](crate::prelude::Event::on_ddui_close) and [`Server::on_ddui_close`](crate::server::Server::on_ddui_close).
*/
pub struct DduiCloseEvent {
    /// Name of the player.
    pub player: String,
    /// Number of the screen, as `DduiHandle::form_id` returns it.
    pub form_id: u32,
    /// Why it closed: `player`, `server`, `busy`, `invalid`, `replaced` or `disconnected`.
    pub reason: String,
}

impl DduiCloseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
