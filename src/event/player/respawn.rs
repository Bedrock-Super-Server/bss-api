use std::cell::Cell;

/**
A player respawns after death.

Handled by [`Event::on_respawn`](crate::prelude::Event::on_respawn).
*/
pub struct RespawnEvent {
    /// Name of the player.
    pub name: String,
    pub(crate) x: Cell<f32>,
    pub(crate) y: Cell<f32>,
    pub(crate) z: Cell<f32>,
}

impl RespawnEvent {
    /// Returns the position the player respawns at.
    pub fn position(&self) -> (f32, f32, f32) {
        (self.x.get(), self.y.get(), self.z.get())
    }

    /// Sets the position the player respawns at.
    pub fn set_position(&self, x: f32, y: f32, z: f32) {
        self.x.set(x);
        self.y.set(y);
        self.z.set(z);
    }
}

impl RespawnEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
