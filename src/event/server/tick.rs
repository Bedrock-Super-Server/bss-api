/**
The server runs a game tick, twenty times per second.

Handled by [`Event::on_tick`](crate::prelude::Event::on_tick).
*/
pub struct TickEvent {
    /// Number of ticks since the server started.
    pub tick: u64,
}
