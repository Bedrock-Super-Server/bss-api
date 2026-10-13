/**
A player answers or closes a form.

Returning `true` from a handler stops other handlers and the form callback from seeing the answer.

Handled by [`Event::on_form_response`](crate::prelude::Event::on_form_response).
*/
pub struct FormResponseEvent {
    /// Name of the player.
    pub name: String,
    /// Id of the form.
    pub form_id: u32,
    /// The JSON answer, or `None` when the player closed the form.
    pub response: Option<String>,
}

impl FormResponseEvent {
    /// Returns `true` when the player closed the form without answering.
    pub fn closed(&self) -> bool {
        self.response.is_none()
    }
}
