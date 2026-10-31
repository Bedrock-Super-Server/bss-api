use std::cell::Cell;

/**
The spawn point of a world is about to move, with `/setworldspawn` or a plugin. Set another position to move it there instead.

`event.cancel()` keeps the spawn point where it is.

Handled by [`Event::on_spawn_change`](crate::prelude::Event::on_spawn_change) and [`Server::on_spawn_change`](crate::server::Server::on_spawn_change).
*/
pub struct SpawnChangeEvent {
    /// X coordinate the spawn point moves to.
    pub(crate) x: Cell<i32>,
    /// Y coordinate the spawn point moves to.
    pub(crate) y: Cell<i32>,
    /// Z coordinate the spawn point moves to.
    pub(crate) z: Cell<i32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl SpawnChangeEvent {
    /// Returns the current value of `x`, which the handlers before this one may have changed.
    ///
    /// X coordinate the spawn point moves to.
    pub fn x(&self) -> i32 {
        self.x.get()
    }

    /// Sets `x`: the server uses the last value set.
    ///
    /// X coordinate the spawn point moves to.
    pub fn set_x(&self, value: i32) {
        self.x.set(value);
    }

    /// Returns the current value of `y`, which the handlers before this one may have changed.
    ///
    /// Y coordinate the spawn point moves to.
    pub fn y(&self) -> i32 {
        self.y.get()
    }

    /// Sets `y`: the server uses the last value set.
    ///
    /// Y coordinate the spawn point moves to.
    pub fn set_y(&self, value: i32) {
        self.y.set(value);
    }

    /// Returns the current value of `z`, which the handlers before this one may have changed.
    ///
    /// Z coordinate the spawn point moves to.
    pub fn z(&self) -> i32 {
        self.z.get()
    }

    /// Sets `z`: the server uses the last value set.
    ///
    /// Z coordinate the spawn point moves to.
    pub fn set_z(&self, value: i32) {
        self.z.set(value);
    }
}
