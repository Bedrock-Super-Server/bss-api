use std::cell::Cell;

/**
A note block plays a note, because a player hit it or it received a redstone signal.

`event.cancel()` keeps the note block silent.

Handled by [`Event::on_note_play`](crate::prelude::Event::on_note_play) and [`Server::on_note_play`](crate::server::Server::on_note_play).
*/
pub struct NotePlayEvent {
    /// Name of the player who hit it, or an empty string when redstone triggered it.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    pub(crate) instrument: Cell<i32>,
    pub(crate) pitch: Cell<i32>,
}

impl NotePlayEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl NotePlayEvent {
    /// Returns the instrument, as the id of the sound the client plays.
    pub fn instrument(&self) -> i32 {
        self.instrument.get()
    }

    /// Sets the instrument, as the id of the sound the client plays.
    pub fn set_instrument(&self, value: i32) {
        self.instrument.set(value);
    }

    /// Returns the pitch, from `0` to `24` semitones above F♯3.
    pub fn pitch(&self) -> i32 {
        self.pitch.get()
    }

    /// Sets the pitch, from `0` to `24` semitones above F♯3.
    pub fn set_pitch(&self, value: i32) {
        self.pitch.set(value);
    }
}
