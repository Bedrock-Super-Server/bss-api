use std::cell::RefCell;

/**
A player sends a chat message.

`event.cancel()` cancels the message: nobody sees it.

Handled by [`Event::on_chat`](crate::prelude::Event::on_chat).
*/
pub struct ChatEvent {
    /// Name of the player as the server knows it.
    pub name: String,
    /// Name the client gave as the sender.
    pub source_name: String,
    /// Text of the message.
    pub message: String,
    /// Kind of message, as the Bedrock protocol numbers it.
    pub message_type: u8,
    /// Xbox user id the client gave.
    pub xbox_user_id: String,
    /// Platform chat id the client gave.
    pub platform_chat_id: String,
    pub(crate) formatted: RefCell<String>,
}

impl ChatEvent {
    /// Returns the line shown to other players, such as `<Steve> hello`.
    pub fn formatted(&self) -> String {
        self.formatted.borrow().clone()
    }

    /// Sets the line shown to other players.
    pub fn set_formatted(&self, message: &str) {
        *self.formatted.borrow_mut() = message.to_string();
    }
}

impl ChatEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
