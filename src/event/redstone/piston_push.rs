use std::cell::Cell;

/**
A piston pushes an entity.

Returning `true` from a handler leaves the entity where it is.

Handled by [`Event::on_piston_push`](crate::prelude::Event::on_piston_push) and [`Server::on_piston_push`](crate::server::Server::on_piston_push).
*/
pub struct PistonPushEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Runtime id of the entity pushed.
    pub entity: u64,
    /// Name of the player pushed, or an empty string for another entity.
    pub player: String,
    pub(crate) motion_x: Cell<f32>,
    pub(crate) motion_y: Cell<f32>,
    pub(crate) motion_z: Cell<f32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl PistonPushEvent {
    /// Returns the motion given along X.
    pub fn motion_x(&self) -> f32 {
        self.motion_x.get()
    }

    /// Sets the motion given along X.
    pub fn set_motion_x(&self, value: f32) {
        self.motion_x.set(value);
    }

    /// Returns the motion given along Y.
    pub fn motion_y(&self) -> f32 {
        self.motion_y.get()
    }

    /// Sets the motion given along Y.
    pub fn set_motion_y(&self, value: f32) {
        self.motion_y.set(value);
    }

    /// Returns the motion given along Z.
    pub fn motion_z(&self) -> f32 {
        self.motion_z.get()
    }

    /// Sets the motion given along Z.
    pub fn set_motion_z(&self, value: f32) {
        self.motion_z.set(value);
    }
}
