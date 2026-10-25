/**
A player's game mode is about to change.

Returning `true` from a handler keeps the current game mode.

Handled by [`Event::on_gamemode_change`](crate::prelude::Event::on_gamemode_change).
*/
pub struct GameModeChangeEvent {
    /// Name of the player.
    pub name: String,
    /// Game mode before the change: `0` survival, `1` creative, `2` adventure, `6` spectator.
    pub from: i32,
    /// Game mode after the change, numbered the same way.
    pub to: i32,
}
