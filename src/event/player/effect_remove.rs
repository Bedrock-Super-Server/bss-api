/**
A player loses a potion effect: it ran out, milk or a command cleared it, or a totem replaced it.

Returning `true` changes nothing: the effect is removed anyway.

Handled by [`Event::on_player_effect_remove`](crate::prelude::Event::on_player_effect_remove) and [`Server::on_player_effect_remove`](crate::server::Server::on_player_effect_remove).
*/
pub struct PlayerEffectRemoveEvent {
    /// Name of the player.
    pub player: String,
    /// Name of the effect, such as `speed`.
    pub effect: String,
    /// Level the effect had, from `0` for level I.
    pub amplifier: u32,
}
