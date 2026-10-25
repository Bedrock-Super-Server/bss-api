use serde_json::{Value, json};

/// What an [`Element`] of a [`DduiScreen`](crate::player::ddui::DduiScreen) is.
#[derive(Clone, Debug, PartialEq)]
pub enum ElementKind {
    /// A title line. It has no value.
    Header {
        /// The text.
        text: String,
    },
    /// A line of text, which a plugin can change while the screen is open.
    Label {
        /// The text.
        text: String,
    },
    /// An empty line.
    Spacer,
    /// A button, which raises [`DduiEvent::Click`](crate::player::ddui::DduiEvent::Click).
    Button {
        /// Text on the button.
        text: String,
    },
    /// An on/off switch.
    Toggle {
        /// Text next to the switch.
        text: String,
        /// State of the switch.
        value: bool,
    },
    /// A slider over whole numbers.
    Slider {
        /// Text above the slider.
        text: String,
        /// Smallest value.
        min: i32,
        /// Largest value.
        max: i32,
        /// Distance between two positions, at least `1`.
        step: i32,
        /// Value of the slider.
        value: i32,
    },
    /// A list to choose one option from.
    Dropdown {
        /// Text above the list.
        text: String,
        /// The options.
        options: Vec<String>,
        /// Index of the option chosen.
        value: usize,
    },
    /// A text field.
    TextField {
        /// Text above the field.
        text: String,
        /// What the field holds.
        value: String,
    },
}

/// One line of a [`DduiScreen`](crate::player::ddui::DduiScreen).
#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    /// Name of the element, which is how events and changes refer to it. Unique in its screen: 1 to
    /// 64 letters, digits, `_`, `-` and `.`.
    pub id: String,
    /// What it is.
    pub kind: ElementKind,
    /// Small grey text under the element, or the tooltip of a button.
    pub description: String,
    /// Whether the player sees it.
    pub visible: bool,
    /// Whether the player cannot use it.
    pub disabled: bool,
}

impl Element {
    pub(crate) fn new(id: impl Into<String>, kind: ElementKind) -> Self {
        Self {
            id: id.into(),
            kind,
            description: String::new(),
            visible: true,
            disabled: false,
        }
    }

    pub(crate) fn json(&self) -> Value {
        let mut value = match &self.kind {
            ElementKind::Header { text } => json!({ "type": "header", "text": text }),
            ElementKind::Label { text } => json!({ "type": "label", "text": text }),
            ElementKind::Spacer => json!({ "type": "spacer" }),
            ElementKind::Button { text } => json!({ "type": "button", "label": text }),
            ElementKind::Toggle { text, value } => {
                json!({ "type": "toggle", "label": text, "value": value })
            }
            ElementKind::Slider {
                text,
                min,
                max,
                step,
                value,
            } => json!({
                "type": "slider", "label": text, "min": min, "max": max, "step": step, "value": value
            }),
            ElementKind::Dropdown {
                text,
                options,
                value,
            } => json!({ "type": "dropdown", "label": text, "options": options, "value": value }),
            ElementKind::TextField { text, value } => {
                json!({ "type": "text_field", "label": text, "text": value })
            }
        };
        value["id"] = json!(self.id);
        value["description"] = json!(self.description);
        value["visible"] = json!(self.visible);
        value["disabled"] = json!(self.disabled);
        value
    }
}
