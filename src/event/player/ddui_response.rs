/**
A player used a control of a data-driven screen: pressed a button, switched a toggle, moved a slider, chose an option or typed in a text field. The server has checked the value.

`event.cancel()` puts the control back as it was, and keeps the handler of the screen from seeing it.

Handled by [`Event::on_ddui_response`](crate::prelude::Event::on_ddui_response) and [`Server::on_ddui_response`](crate::server::Server::on_ddui_response).
*/
pub struct DduiResponseEvent {
    /// Name of the player.
    pub player: String,
    /// Number of the screen, as `DduiHandle::form_id` returns it.
    pub form_id: u32,
    /// Id of the element the player used.
    pub element: String,
    /// What the element is: `click`, `toggle`, `slider`, `select` or `text`.
    pub kind: String,
    /// The value of a slider, or the index of the option chosen in a dropdown; `0` otherwise.
    pub number: i32,
    /// The state of a toggle after the player switched it; `false` otherwise.
    pub flag: bool,
    /// What a text field holds after the player typed; empty otherwise.
    pub text: String,
}

impl DduiResponseEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}
