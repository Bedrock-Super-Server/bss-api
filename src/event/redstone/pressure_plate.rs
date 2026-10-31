use std::cell::Cell;

/**
Something steps on or leaves a pressure plate.

`event.cancel()` keeps the plate in its previous state.

Handled by [`Event::on_pressure_plate`](crate::prelude::Event::on_pressure_plate) and [`Server::on_pressure_plate`](crate::server::Server::on_pressure_plate).
*/
pub struct PressurePlateEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) signal: Cell<i32>,
    /// Name of the player on the plate, or an empty string.
    pub player: String,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl PressurePlateEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl PressurePlateEvent {
    /// Returns the redstone signal the plate emits, from `0` to `15`.
    pub fn signal(&self) -> i32 {
        self.signal.get()
    }

    /// Sets the redstone signal the plate emits, from `0` to `15`.
    pub fn set_signal(&self, value: i32) {
        self.signal.set(value);
    }
}
