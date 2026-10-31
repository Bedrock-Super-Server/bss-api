use std::cell::Cell;

/**
A player finishes eating or drinking an item.

`event.cancel()` cancels the food the item gives.

Handled by [`Event::on_consume`](crate::prelude::Event::on_consume).
*/
pub struct ConsumeEvent {
    /// Name of the player.
    pub name: String,
    /// Numeric id of the item consumed.
    pub item_id: i16,
    pub(crate) food_points: Cell<u32>,
    pub(crate) saturation: Cell<f32>,
}

impl ConsumeEvent {
    /// Returns the hunger points the item restores.
    pub fn food_points(&self) -> u32 {
        self.food_points.get()
    }

    /// Sets the hunger points the item restores.
    pub fn set_food_points(&self, points: u32) {
        self.food_points.set(points);
    }

    /// Returns the saturation the item gives.
    pub fn saturation(&self) -> f32 {
        self.saturation.get()
    }

    /// Sets the saturation the item gives.
    pub fn set_saturation(&self, saturation: f32) {
        self.saturation.set(saturation);
    }
}

impl ConsumeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
