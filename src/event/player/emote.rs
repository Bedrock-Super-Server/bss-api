use std::cell::RefCell;

/**
A player plays an emote.

Returning `true` from a handler hides the emote from other players.

Handled by [`Event::on_emote`](crate::prelude::Event::on_emote).
*/
pub struct EmoteEvent {
    /// Name of the player.
    pub name: String,
    /// Runtime id of the player.
    pub entity_id: u64,
    /// How long the emote plays, in ticks.
    pub duration: u32,
    /// Xbox user id of the player.
    pub xuid: String,
    /// Platform id the client gave.
    pub platform_id: String,
    /// Flags the client sent with the emote.
    pub flags: u8,
    pub(crate) emote_id: RefCell<String>,
}

impl EmoteEvent {
    /// Returns the id of the emote shown to other players.
    pub fn emote_id(&self) -> String {
        self.emote_id.borrow().clone()
    }

    /// Sets the id of the emote shown to other players.
    pub fn set_emote_id(&self, emote_id: &str) {
        *self.emote_id.borrow_mut() = emote_id.to_string();
    }
}
