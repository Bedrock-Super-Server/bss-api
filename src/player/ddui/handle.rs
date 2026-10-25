use serde_json::{Value, json};

use crate::ffi::abi::FfiStr;
use crate::player::Player;
use crate::player::ddui::{DduiScreen, handlers};

/**
An open data-driven screen: what [`Player::send_ddui`](crate::player::Player::send_ddui) returns, and what its
handler receives with each event.

A `DduiHandle` is a cheap handle: copy it freely. Once the screen has closed, every method does nothing and
returns `false`.

Every change is checked by the server: an element that does not exist, a value of another type than the
element's, a number outside a slider or an option that does not exist is refused, and the method returns
`false`.
*/
#[derive(Clone, Copy)]
pub struct DduiHandle {
    player: Player,
    form_id: u32,
}

impl DduiHandle {
    pub(crate) fn new(player: Player, form_id: u32) -> Self {
        Self { player, form_id }
    }

    /// Returns the player who has the screen.
    pub fn player(&self) -> Player {
        self.player
    }

    /// Returns the number of the screen, which the events of [`DduiResponseEvent`](crate::event::DduiResponseEvent) carry.
    pub fn form_id(&self) -> u32 {
        self.form_id
    }

    /// Returns whether the screen is still open.
    pub fn is_open(&self) -> bool {
        handlers::is_open(self.player.handle, self.form_id)
    }

    /// Changes the text of a label or a header, or what a text field holds.
    pub fn set_text(&self, id: &str, text: &str) -> bool {
        self.update(id, "text", json!(text))
    }

    /// Changes the text of a button, or the text above a toggle, a slider, a dropdown or a text field.
    pub fn set_label(&self, id: &str, text: &str) -> bool {
        self.update(id, "label", json!(text))
    }

    /// Changes the small grey text under an element, or the tooltip of a button.
    pub fn set_description(&self, id: &str, text: &str) -> bool {
        self.update(id, "description", json!(text))
    }

    /// Shows or hides an element.
    pub fn set_visible(&self, id: &str, visible: bool) -> bool {
        self.update(id, "visible", json!(visible))
    }

    /// Enables or disables an element.
    pub fn set_disabled(&self, id: &str, disabled: bool) -> bool {
        self.update(id, "disabled", json!(disabled))
    }

    /// Switches a toggle.
    pub fn set_toggled(&self, id: &str, toggled: bool) -> bool {
        self.update(id, "value", json!(toggled))
    }

    /// Moves a slider, within its range.
    pub fn set_slider(&self, id: &str, value: i32) -> bool {
        self.update(id, "value", json!(value))
    }

    /// Chooses the option at `index` of a dropdown.
    pub fn set_selected(&self, id: &str, index: usize) -> bool {
        self.update(id, "value", json!(index))
    }

    /// Replaces everything the screen shows with `screen`, keeping the same window open.
    pub fn replace(&self, screen: &DduiScreen) -> bool {
        let json = screen.json();
        host_call!(
            self.player.host,
            ddui_replace(self.player.handle, self.form_id, FfiStr::new(&json))
        )
        .unwrap_or(false)
    }

    /// Closes the screen. Its handler gets no more events.
    pub fn close(&self) -> bool {
        let closed = host_call!(
            self.player.host,
            ddui_close(self.player.handle, self.form_id)
        )
        .unwrap_or(false);
        handlers::forget(self.player.handle, self.form_id);
        closed
    }

    fn update(&self, id: &str, field: &str, value: Value) -> bool {
        let value = value.to_string();
        host_call!(
            self.player.host,
            ddui_update(
                self.player.handle,
                self.form_id,
                FfiStr::new(id),
                FfiStr::new(field),
                FfiStr::new(&value),
            )
        )
        .unwrap_or(false)
    }
}
