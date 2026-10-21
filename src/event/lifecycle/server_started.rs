/**
The server finished starting and now accepts players. `worlds` is how many worlds are loaded.

Returning `true` changes nothing.

Handled by [`Event::on_server_started`](crate::prelude::Event::on_server_started) and [`Server::on_server_started`](crate::server::Server::on_server_started).
*/
pub struct ServerStartedEvent {
    /// How many worlds are loaded.
    pub worlds: u32,
}
