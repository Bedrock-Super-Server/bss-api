use std::cell::Cell;

/**
An entity other than a player goes through a portal.

Returning `true` from a handler keeps the entity where it is.

Handled by [`Event::on_entity_portal`](crate::prelude::Event::on_entity_portal) and [`Server::on_entity_portal`](crate::server::Server::on_entity_portal).
*/
pub struct EntityPortalEvent {
    /// Runtime id of the entity.
    pub entity: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// Kind of portal: `nether`, `end` or `gateway`.
    pub portal: String,
    /// Id of the world the entity leaves.
    pub from_world: u32,
    /// Id of the world the entity arrives in.
    pub to_world: u32,
    pub(crate) x: Cell<f32>,
    pub(crate) y: Cell<f32>,
    pub(crate) z: Cell<f32>,
}

impl EntityPortalEvent {
    /// Returns the X coordinate of the arrival.
    pub fn x(&self) -> f32 {
        self.x.get()
    }

    /// Sets the X coordinate of the arrival.
    pub fn set_x(&self, value: f32) {
        self.x.set(value);
    }

    /// Returns the Y coordinate of the arrival.
    pub fn y(&self) -> f32 {
        self.y.get()
    }

    /// Sets the Y coordinate of the arrival.
    pub fn set_y(&self, value: f32) {
        self.y.set(value);
    }

    /// Returns the Z coordinate of the arrival.
    pub fn z(&self) -> f32 {
        self.z.get()
    }

    /// Sets the Z coordinate of the arrival.
    pub fn set_z(&self, value: f32) {
        self.z.set(value);
    }
}
