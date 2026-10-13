/**
The weather changes.

Handled by [`Event::on_weather_change`](crate::prelude::Event::on_weather_change).
*/
pub struct WeatherChangeEvent {
    /// New weather; [`is_clear`](Self::is_clear), [`is_raining`](Self::is_raining) and [`is_thundering`](Self::is_thundering) tell it apart.
    pub kind: u8,
    /// Intensity of the rain, from `0.0` to `1.0`.
    pub level: f32,
}

impl WeatherChangeEvent {
    /// Returns `true` when the weather turns clear.
    pub fn is_clear(&self) -> bool {
        self.kind == 0
    }

    /// Returns `true` when rain starts.
    pub fn is_raining(&self) -> bool {
        self.kind == 1
    }

    /// Returns `true` when a thunderstorm starts.
    pub fn is_thundering(&self) -> bool {
        self.kind == 2
    }
}
