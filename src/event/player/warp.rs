use std::cell::Cell;

/**
A player teleports with an ender pearl or a chorus fruit.

Returning `true` from a handler cancels the teleport.

Handled by [`Event::on_player_warp`](crate::prelude::Event::on_player_warp) and [`Server::on_player_warp`](crate::server::Server::on_player_warp).
*/
pub struct PlayerWarpEvent {
    /// Name of the player.
    pub player: String,
    /// `ender_pearl` or `chorus_fruit`.
    pub cause: String,
    pub(crate) x: Cell<f32>,
    pub(crate) y: Cell<f32>,
    pub(crate) z: Cell<f32>,
}

impl PlayerWarpEvent {
    /// Returns the X coordinate of the destination.
    pub fn x(&self) -> f32 {
        self.x.get()
    }

    /// Sets the X coordinate of the destination.
    pub fn set_x(&self, value: f32) {
        self.x.set(value);
    }

    /// Returns the Y coordinate of the destination.
    pub fn y(&self) -> f32 {
        self.y.get()
    }

    /// Sets the Y coordinate of the destination.
    pub fn set_y(&self, value: f32) {
        self.y.set(value);
    }

    /// Returns the Z coordinate of the destination.
    pub fn z(&self) -> f32 {
        self.z.get()
    }

    /// Sets the Z coordinate of the destination.
    pub fn set_z(&self, value: f32) {
        self.z.set(value);
    }
}
