use std::cell::Cell;

/**
A player is about to get a potion effect, from a potion, food, a totem, a command or a plugin; `effect` is its name, such as `speed`.

Returning `true` from a handler keeps the player from getting the effect.

Handled by [`Event::on_player_effect_add`](crate::prelude::Event::on_player_effect_add) and [`Server::on_player_effect_add`](crate::server::Server::on_player_effect_add).
*/
pub struct PlayerEffectAddEvent {
    /// Name of the player.
    pub player: String,
    /// Name of the effect, such as `speed`.
    pub effect: String,
    /// Level of the effect, from `0` for level I.
    pub(crate) amplifier: Cell<u32>,
    /// How long it lasts, in ticks.
    pub(crate) duration: Cell<i32>,
}

impl PlayerEffectAddEvent {
    /// Returns the current value of `amplifier`, which the handlers before this one may have changed.
    ///
    /// Level of the effect, from `0` for level I.
    pub fn amplifier(&self) -> u32 {
        self.amplifier.get()
    }

    /// Sets `amplifier`: the server uses the last value set.
    ///
    /// Level of the effect, from `0` for level I.
    pub fn set_amplifier(&self, value: u32) {
        self.amplifier.set(value);
    }

    /// Returns the current value of `duration`, which the handlers before this one may have changed.
    ///
    /// How long it lasts, in ticks.
    pub fn duration(&self) -> i32 {
        self.duration.get()
    }

    /// Sets `duration`: the server uses the last value set.
    ///
    /// How long it lasts, in ticks.
    pub fn set_duration(&self, value: i32) {
        self.duration.set(value);
    }
}
