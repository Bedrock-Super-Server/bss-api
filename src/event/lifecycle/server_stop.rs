/**
The server begins to stop, before it disconnects the players, unloads the plugins and saves the worlds. `players` is how many are online.

`event.cancel()` does not keep the server from stopping.

Handled by [`Event::on_server_stop`](crate::prelude::Event::on_server_stop) and [`Server::on_server_stop`](crate::server::Server::on_server_stop).
*/
pub struct ServerStopEvent {
    /// How many players are online.
    pub players: u32,
}
