use std::cell::Cell;

/**
A player's hunger bar changes.

`event.cancel()` keeps the previous values.

Handled by [`Event::on_food_change`](crate::prelude::Event::on_food_change) and [`Server::on_food_change`](crate::server::Server::on_food_change).
*/
pub struct FoodChangeEvent {
    /// Name of the player.
    pub player: String,
    /// Hunger points before the change, from `0` to `20`.
    pub old_food: i32,
    pub(crate) food: Cell<i32>,
    pub(crate) saturation: Cell<f32>,
}

impl FoodChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl FoodChangeEvent {
    /// Returns the hunger points after the change, from `0` to `20`.
    pub fn food(&self) -> i32 {
        self.food.get()
    }

    /// Sets the hunger points after the change, from `0` to `20`.
    pub fn set_food(&self, value: i32) {
        self.food.set(value);
    }

    /// Returns the saturation after the change.
    pub fn saturation(&self) -> f32 {
        self.saturation.get()
    }

    /// Sets the saturation after the change.
    pub fn set_saturation(&self, value: f32) {
        self.saturation.set(value);
    }
}
