use std::cell::RefCell;

/**
A player finishes editing the text of a sign.

`event.cancel()` leaves the sign unchanged.

Handled by [`Event::on_sign_change`](crate::prelude::Event::on_sign_change) and [`Server::on_sign_change`](crate::server::Server::on_sign_change).
*/
pub struct SignChangeEvent {
    /// Name of the player.
    pub player: String,
    /// X coordinate of the block.
    pub x: i32,
    /// Y coordinate of the block.
    pub y: i32,
    /// Z coordinate of the block.
    pub z: i32,
    /// `true` for the front of the sign, `false` for the back.
    pub front: bool,
    pub(crate) text: RefCell<String>,
}

impl SignChangeEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl SignChangeEvent {
    /// Returns the text written on the sign, one line per `\n`.
    pub fn text(&self) -> String {
        self.text.borrow().clone()
    }

    /// Sets the text written on the sign, one line per `\n`.
    pub fn set_text(&self, value: impl Into<String>) {
        *self.text.borrow_mut() = value.into();
    }
}
