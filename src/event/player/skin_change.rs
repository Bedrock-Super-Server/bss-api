/**
A player changes skin while in game.

`event.cancel()` keeps the previous skin.

Handled by [`Event::on_skin_change`](crate::prelude::Event::on_skin_change).
*/
pub struct SkinChangeEvent {
    /// Name of the player.
    pub name: String,
    /// The new skin, in the Bedrock serialized skin format.
    pub skin: Vec<u8>,
    /// Name of the new skin.
    pub new_skin_name: String,
    /// Name of the previous skin.
    pub old_skin_name: String,
}

impl SkinChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
