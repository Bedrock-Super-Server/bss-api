/*!
Forms: the menus, dialogs and settings screens the Bedrock client draws.

Build a [`Form`] and show it with [`Player::send_form`](crate::player::Player::send_form). The handler receives the player's answer as a [`FormResponse`].

| Form | Shows | Answer |
|---|---|---|
| [`SimpleForm`] | a text and a list of buttons | [`FormResponse::Button`] |
| [`ModalForm`] | a text and two buttons | [`FormResponse::Confirmed`] |
| [`CustomForm`] | inputs, toggles, sliders, dropdowns | [`FormResponse::Values`] |

# Example

```
use bss_api::prelude::*;

fn ask_name(player: &Player) {
    let form = Form::Custom(CustomForm {
        title: "Profile".into(),
        elements: vec![
            Element::Input { label: "Nickname".into(), placeholder: "Steve".into(), default: String::new() },
            Element::Toggle { label: "Show in the tab list".into(), default: true },
        ],
    });
    let who = *player;
    player.send_form(form, move |response| {
        let FormResponse::Values(values) = response else { return };
        let nickname = values[0].as_input().unwrap_or_default();
        let listed = values[1].as_toggle().unwrap_or(true);
        who.message(&format!("Saved {nickname} (listed: {listed})"));
    });
}
```
*/

use serde_json::{Value, json};

/// The value of one [`Element`] of a [`CustomForm`], in the same position.
pub enum FormValue {
    /// A [`Element::Label`], which has no value.
    Label,
    /// The text of an [`Element::Input`].
    Input(String),
    /// The state of an [`Element::Toggle`].
    Toggle(bool),
    /// The value of an [`Element::Slider`].
    Slider(f32),
    /// The index of the option chosen in an [`Element::Dropdown`].
    Dropdown(usize),
    /// The index of the step chosen in an [`Element::StepSlider`].
    StepSlider(usize),
}

impl FormValue {
    /// Returns the text of an input.
    pub fn as_input(&self) -> Option<&str> {
        match self {
            FormValue::Input(text) => Some(text),
            _ => None,
        }
    }

    /// Returns the state of a toggle.
    pub fn as_toggle(&self) -> Option<bool> {
        match self {
            FormValue::Toggle(flag) => Some(*flag),
            _ => None,
        }
    }

    /// Returns the index chosen in a dropdown or a step slider.
    pub fn as_index(&self) -> Option<usize> {
        match self {
            FormValue::Dropdown(index) | FormValue::StepSlider(index) => Some(*index),
            _ => None,
        }
    }
}

/// An image shown on a [`Button`].
pub enum Image {
    /// An image downloaded from a URL.
    Url(String),
    /// A texture of a resource pack, such as `textures/items/diamond`.
    Path(String),
}

/// A button of a [`SimpleForm`].
pub struct Button {
    /// Text of the button.
    pub text: String,
    /// Image shown on the button.
    pub image: Option<Image>,
}

impl Button {
    /// A button with text only.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            image: None,
        }
    }

    /// A button with an image downloaded from `url`.
    pub fn with_url(text: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            image: Some(Image::Url(url.into())),
        }
    }

    /// A button with a resource pack texture, such as `textures/items/diamond`.
    pub fn with_path(text: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            image: Some(Image::Path(path.into())),
        }
    }
}

/**
A text and a list of buttons. The answer is the index of the button clicked.

```
use bss_api::prelude::*;

let menu = Form::Simple(SimpleForm {
    title: "Warps".into(),
    body: "Where to?".into(),
    buttons: vec![Button::new("Spawn"), Button::with_path("Mine", "textures/items/iron_pickaxe")],
});
# let _ = menu;
```
*/
pub struct SimpleForm {
    /// Title of the window.
    pub title: String,
    /// Text above the buttons.
    pub body: String,
    /// Buttons, from top to bottom.
    pub buttons: Vec<Button>,
}

/// A question with two answers, such as yes and no.
pub struct ModalForm {
    /// Title of the window.
    pub title: String,
    /// The question.
    pub body: String,
    /// Text of the first button, which answers `true`.
    pub confirm: String,
    /// Text of the second button, which answers `false`.
    pub cancel: String,
}

/// One line of a [`CustomForm`].
pub enum Element {
    /// A line of text.
    Label {
        /// The text.
        text: String,
    },
    /// A text field.
    Input {
        /// Text above the field.
        label: String,
        /// Grey text shown while the field is empty.
        placeholder: String,
        /// Initial text.
        default: String,
    },
    /// An on/off switch.
    Toggle {
        /// Text next to the switch.
        label: String,
        /// Initial state.
        default: bool,
    },
    /// A slider over a range of numbers.
    Slider {
        /// Text above the slider.
        label: String,
        /// Smallest value.
        min: f32,
        /// Largest value.
        max: f32,
        /// Distance between two positions.
        step: f32,
        /// Initial value.
        default: f32,
    },
    /// A list to choose one option from.
    Dropdown {
        /// Text above the list.
        label: String,
        /// The options.
        options: Vec<String>,
        /// Index of the option chosen at first.
        default: usize,
    },
    /// A slider over named steps.
    StepSlider {
        /// Text above the slider.
        label: String,
        /// The steps.
        steps: Vec<String>,
        /// Index of the step chosen at first.
        default: usize,
    },
}

/// A form with fields to fill in. The answer holds one [`FormValue`] per element, in the same order.
pub struct CustomForm {
    /// Title of the window.
    pub title: String,
    /// Fields, from top to bottom.
    pub elements: Vec<Element>,
}

/// A form to show with [`Player::send_form`](crate::player::Player::send_form).
pub enum Form {
    /// A list of buttons.
    Simple(SimpleForm),
    /// A question with two answers.
    Modal(ModalForm),
    /// Fields to fill in.
    Custom(CustomForm),
}

/// A player's answer to a [`Form`].
pub enum FormResponse {
    /// Index of the button clicked in a [`SimpleForm`].
    Button(usize),
    /// Answer to a [`ModalForm`]: `true` for its first button.
    Confirmed(bool),
    /// Values of the fields of a [`CustomForm`].
    Values(Vec<FormValue>),
    /// The player closed the form without answering.
    Closed,
}

impl Form {
    pub(crate) fn to_json(&self) -> String {
        let value = match self {
            Form::Simple(form) => json!({
                "type": "form",
                "title": form.title,
                "content": form.body,
                "buttons": form.buttons.iter().map(button_json).collect::<Vec<_>>(),
            }),
            Form::Modal(form) => json!({
                "type": "modal",
                "title": form.title,
                "content": form.body,
                "button1": form.confirm,
                "button2": form.cancel,
            }),
            Form::Custom(form) => json!({
                "type": "custom_form",
                "title": form.title,
                "content": form.elements.iter().map(element_json).collect::<Vec<_>>(),
            }),
        };
        value.to_string()
    }

    pub(crate) fn parse(&self, raw: &str) -> FormResponse {
        /* The client answers with bare JSON: `null` when the form was closed,
         * the index of a button for a simple form, a boolean for a modal one,
         * and an array with one value per element for a custom form. Anything
         * that does not match the form that was sent is treated as closed
         * rather than guessed at. */
        let raw = raw.trim();
        if raw.is_empty() || raw == "null" {
            return FormResponse::Closed;
        }
        match self {
            Form::Simple(_) => serde_json::from_str::<usize>(raw)
                .map(FormResponse::Button)
                .unwrap_or(FormResponse::Closed),
            Form::Modal(_) => serde_json::from_str::<bool>(raw)
                .map(FormResponse::Confirmed)
                .unwrap_or(FormResponse::Closed),
            Form::Custom(form) => parse_custom_values(&form.elements, raw)
                .map(FormResponse::Values)
                .unwrap_or(FormResponse::Closed),
        }
    }
}

fn button_json(button: &Button) -> Value {
    match &button.image {
        Some(Image::Url(url)) => {
            json!({ "text": button.text, "image": { "type": "url", "data": url } })
        }
        Some(Image::Path(path)) => {
            json!({ "text": button.text, "image": { "type": "path", "data": path } })
        }
        None => json!({ "text": button.text }),
    }
}

fn element_json(element: &Element) -> Value {
    match element {
        Element::Label { text } => json!({ "type": "label", "text": text }),
        Element::Input {
            label,
            placeholder,
            default,
        } => {
            json!({ "type": "input", "text": label, "placeholder": placeholder, "default_text": default })
        }
        Element::Toggle { label, default } => {
            json!({ "type": "toggle", "text": label, "default_bool": default })
        }
        Element::Slider {
            label,
            min,
            max,
            step,
            default,
        } => {
            json!({ "type": "slider", "text": label, "min": min, "max": max, "step": step, "default_value": default })
        }
        Element::Dropdown {
            label,
            options,
            default,
        } => {
            json!({ "type": "dropdown", "text": label, "options": options, "default_index": default })
        }
        Element::StepSlider {
            label,
            steps,
            default,
        } => {
            json!({ "type": "step_slider", "text": label, "options": steps, "default_index": default })
        }
    }
}

fn parse_custom_values(elements: &[Element], raw: &str) -> Option<Vec<FormValue>> {
    let values: Vec<Value> = serde_json::from_str(raw).ok()?;
    if values.len() != elements.len() {
        return None;
    }
    let mut out = Vec::with_capacity(elements.len());
    for (element, value) in elements.iter().zip(values) {
        out.push(match element {
            Element::Label { .. } => FormValue::Label,
            Element::Input { .. } => {
                FormValue::Input(value.as_str().unwrap_or_default().to_string())
            }
            Element::Toggle { .. } => FormValue::Toggle(value.as_bool().unwrap_or(false)),
            Element::Slider { .. } => FormValue::Slider(value.as_f64().unwrap_or(0.0) as f32),
            Element::Dropdown { .. } => FormValue::Dropdown(value.as_u64().unwrap_or(0) as usize),
            Element::StepSlider { .. } => {
                FormValue::StepSlider(value.as_u64().unwrap_or(0) as usize)
            }
        });
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn custom() -> Form {
        Form::Custom(CustomForm {
            title: "t".into(),
            elements: vec![
                Element::Label { text: "hi".into() },
                Element::Input {
                    label: "name".into(),
                    placeholder: String::new(),
                    default: String::new(),
                },
                Element::Toggle {
                    label: "on".into(),
                    default: false,
                },
                Element::Dropdown {
                    label: "pick".into(),
                    options: vec!["a".into(), "b".into()],
                    default: 0,
                },
            ],
        })
    }

    #[test]
    fn a_closed_form_answers_closed() {
        assert!(matches!(custom().parse("null"), FormResponse::Closed));
        assert!(matches!(custom().parse(""), FormResponse::Closed));
    }

    #[test]
    fn custom_values_keep_the_order_of_the_elements() {
        let FormResponse::Values(values) = custom().parse(r#"[null, "Alex", true, 1]"#) else {
            panic!("not values")
        };
        assert!(matches!(values[0], FormValue::Label));
        assert_eq!(values[1].as_input(), Some("Alex"));
        assert_eq!(values[2].as_toggle(), Some(true));
        assert_eq!(values[3].as_index(), Some(1));
    }

    #[test]
    fn an_answer_of_the_wrong_length_is_treated_as_closed() {
        assert!(matches!(
            custom().parse("[null, \"Alex\"]"),
            FormResponse::Closed
        ));
    }

    #[test]
    fn simple_and_modal_answers_parse() {
        let simple = Form::Simple(SimpleForm {
            title: String::new(),
            body: String::new(),
            buttons: vec![Button::new("a"), Button::new("b")],
        });
        assert!(matches!(simple.parse("1"), FormResponse::Button(1)));
        let modal = Form::Modal(ModalForm {
            title: String::new(),
            body: String::new(),
            confirm: "yes".into(),
            cancel: "no".into(),
        });
        assert!(matches!(
            modal.parse("false"),
            FormResponse::Confirmed(false)
        ));
    }

    #[test]
    fn a_simple_form_serializes_as_the_client_expects() {
        let form = Form::Simple(SimpleForm {
            title: "T".into(),
            body: "B".into(),
            buttons: vec![Button::with_url("Go", "https://x/y.png")],
        });
        let json: Value = serde_json::from_str(&form.to_json()).unwrap();
        assert_eq!(json["type"], "form");
        assert_eq!(json["buttons"][0]["image"]["type"], "url");
    }
}
