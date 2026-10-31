/**
A player's client reports on the resource packs of the server.

`event.cancel()` changes nothing: a client that refuses the packs is disconnected anyway.

Handled by [`Event::on_resource_pack_status`](crate::prelude::Event::on_resource_pack_status) and [`Server::on_resource_pack_status`](crate::server::Server::on_resource_pack_status).
*/
pub struct ResourcePackStatusEvent {
    /// Name of the player.
    pub player: String,
    /// `downloading`, `downloaded`, `loaded` or `refused`.
    pub status: String,
}

impl ResourcePackStatusEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
