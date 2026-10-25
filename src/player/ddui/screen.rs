use serde_json::{Value, json};

use crate::player::ddui::element::{Element, ElementKind};

/**
A data-driven screen: a window that stays open while the plugin and the player change what it shows.

Build it with the methods below, one line after the other, and show it with
[`Player::send_ddui`](crate::player::Player::send_ddui). Every element has an `id`, and
[`DduiEvent`](crate::player::ddui::DduiEvent)s and [`DduiHandle`](crate::player::ddui::DduiHandle) changes
refer to it. A modifier such as [`description`](DduiScreen::description) applies to the element added last.

The server refuses a screen with more than 128 elements, two elements with the same id, an id that is
not 1 to 64 letters, digits, `_`, `-` or `.`, a slider whose `min` is above its `max` or whose `step`
is below `1`, more than 64 options in a dropdown, or a text longer than 1000 characters.

# Example

```
use bss_api::prelude::*;

fn open(player: &Player) {
    let screen = DduiScreen::new("Settings")
        .header("title", "Profile")
        .text_field("name", "Nickname", "")
        .toggle("notify", "Notifications", true)
        .description("Sent to you in the chat")
        .slider("volume", "Volume", 0, 100, 5, 50)
        .dropdown("mode", "Mode", &["Easy", "Normal", "Hard"], 1)
        .button("save", "Save");
    player.send_ddui(screen, |handle, event| {
        if let DduiEvent::Click { id } = event {
            if id == "save" {
                handle.close();
            }
        }
    });
}
```
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DduiScreen {
    title: String,
    elements: Vec<Element>,
}

impl DduiScreen {
    /// A screen with a title and nothing in it.
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            elements: Vec::new(),
        }
    }

    /// Adds a title line.
    pub fn header(self, id: impl Into<String>, text: impl Into<String>) -> Self {
        self.add(id, ElementKind::Header { text: text.into() })
    }

    /// Adds a line of text.
    pub fn label(self, id: impl Into<String>, text: impl Into<String>) -> Self {
        self.add(id, ElementKind::Label { text: text.into() })
    }

    /// Adds an empty line.
    pub fn spacer(self, id: impl Into<String>) -> Self {
        self.add(id, ElementKind::Spacer)
    }

    /// Adds a button.
    pub fn button(self, id: impl Into<String>, text: impl Into<String>) -> Self {
        self.add(id, ElementKind::Button { text: text.into() })
    }

    /// Adds an on/off switch.
    pub fn toggle(self, id: impl Into<String>, text: impl Into<String>, value: bool) -> Self {
        self.add(
            id,
            ElementKind::Toggle {
                text: text.into(),
                value,
            },
        )
    }

    /// Adds a slider over whole numbers from `min` to `max`, moving by `step`.
    pub fn slider(
        self,
        id: impl Into<String>,
        text: impl Into<String>,
        min: i32,
        max: i32,
        step: i32,
        value: i32,
    ) -> Self {
        self.add(
            id,
            ElementKind::Slider {
                text: text.into(),
                min,
                max,
                step,
                value,
            },
        )
    }

    /// Adds a list of `options`, with the one at index `value` chosen.
    pub fn dropdown(
        self,
        id: impl Into<String>,
        text: impl Into<String>,
        options: &[&str],
        value: usize,
    ) -> Self {
        self.add(
            id,
            ElementKind::Dropdown {
                text: text.into(),
                options: options.iter().map(|option| option.to_string()).collect(),
                value,
            },
        )
    }

    /// Adds a text field that starts with `value`.
    pub fn text_field(
        self,
        id: impl Into<String>,
        text: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.add(
            id,
            ElementKind::TextField {
                text: text.into(),
                value: value.into(),
            },
        )
    }

    /// Sets the small grey text under the element added last, or the tooltip of a button.
    pub fn description(mut self, text: impl Into<String>) -> Self {
        if let Some(element) = self.elements.last_mut() {
            element.description = text.into();
        }
        self
    }

    /// Hides the element added last until [`DduiHandle::set_visible`](crate::player::ddui::DduiHandle::set_visible) shows it.
    pub fn hidden(mut self) -> Self {
        if let Some(element) = self.elements.last_mut() {
            element.visible = false;
        }
        self
    }

    /// Disables the element added last until [`DduiHandle::set_disabled`](crate::player::ddui::DduiHandle::set_disabled) enables it.
    pub fn disabled(mut self) -> Self {
        if let Some(element) = self.elements.last_mut() {
            element.disabled = true;
        }
        self
    }

    /// The elements, from top to bottom.
    pub fn elements(&self) -> &[Element] {
        &self.elements
    }

    fn add(mut self, id: impl Into<String>, kind: ElementKind) -> Self {
        self.elements.push(Element::new(id, kind));
        self
    }

    pub(crate) fn json(&self) -> String {
        let elements: Vec<Value> = self.elements.iter().map(Element::json).collect();
        json!({ "title": self.title, "elements": elements }).to_string()
    }
}
