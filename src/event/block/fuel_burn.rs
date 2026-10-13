use std::cell::Cell;

/**
A furnace, smoker or blast furnace starts burning a fuel item.

Returning `true` from a handler keeps the fuel unburnt.

Handled by [`Event::on_fuel_burn`](crate::prelude::Event::on_fuel_burn) and [`Server::on_fuel_burn`](crate::server::Server::on_fuel_burn).
*/
pub struct FuelBurnEvent {
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// Kind of station, such as `furnace`, `smoker` or `blast_furnace`.
    pub station: String,
    /// Numeric id of the fuel item.
    pub fuel: i16,
    pub(crate) burn_ticks: Cell<u32>,
    /// Id of the world, in load order: the default world is `0`.
    pub world: u32,
}

impl FuelBurnEvent {
    /// Returns how long the fuel burns, in ticks.
    pub fn burn_ticks(&self) -> u32 {
        self.burn_ticks.get()
    }

    /// Sets how long the fuel burns, in ticks.
    pub fn set_burn_ticks(&self, value: u32) {
        self.burn_ticks.set(value);
    }
}
