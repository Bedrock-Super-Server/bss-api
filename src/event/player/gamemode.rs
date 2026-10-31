/**
A player's game mode is about to change.

`event.cancel()` keeps the current game mode.

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

impl GameModeChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
