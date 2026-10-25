use serde_json::Value;

/// Why a data-driven screen closed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DduiClose {
    /// The player closed it.
    Player,
    /// The server closed it by itself.
    Server,
    /// It never opened: the player had another screen open, such as the chat.
    Busy,
    /// The client refused its content.
    Invalid,
    /// A screen shown to the same player took its place.
    Replaced,
    /// The player left the server.
    Disconnected,
    /// Another reason, which this version of the API does not know.
    Other,
}

/// What happened in a data-driven screen, as [`Player::send_ddui`](crate::player::Player::send_ddui) hands it to its handler.
///
/// The server has already checked every value: an element the player cannot see or use, a number
/// outside a slider, an option that does not exist or a text that is too long never get here.
#[derive(Clone, Debug, PartialEq)]
pub enum DduiEvent {
    /// The player pressed the button `id`.
    Click {
        /// Id of the button.
        id: String,
    },
    /// The player switched the toggle `id`.
    Toggle {
        /// Id of the toggle.
        id: String,
        /// The new state.
        value: bool,
    },
    /// The player moved the slider `id`.
    Slider {
        /// Id of the slider.
        id: String,
        /// The new value, within the slider.
        value: i32,
    },
    /// The player chose an option of the dropdown `id`.
    Select {
        /// Id of the dropdown.
        id: String,
        /// Index of the option.
        index: usize,
    },
    /// The player typed in the text field `id`.
    Text {
        /// Id of the text field.
        id: String,
        /// What the field holds, at most 500 characters.
        value: String,
    },
    /// The screen closed. It is the last event of the screen.
    Closed(DduiClose),
}

impl DduiEvent {
    pub(crate) fn parse(value: &Value) -> Option<Self> {
        let id = || value["id"].as_str().map(str::to_string);
        Some(match value["event"].as_str()? {
            "click" => Self::Click { id: id()? },
            "toggle" => Self::Toggle {
                id: id()?,
                value: value["value"].as_bool()?,
            },
            "slider" => Self::Slider {
                id: id()?,
                value: i32::try_from(value["value"].as_i64()?).ok()?,
            },
            "select" => Self::Select {
                id: id()?,
                index: usize::try_from(value["value"].as_u64()?).ok()?,
            },
            "text" => Self::Text {
                id: id()?,
                value: value["value"].as_str()?.to_string(),
            },
            "close" => Self::Closed(match value["reason"].as_str()? {
                "player" => DduiClose::Player,
                "server" => DduiClose::Server,
                "busy" => DduiClose::Busy,
                "invalid" => DduiClose::Invalid,
                "replaced" => DduiClose::Replaced,
                "disconnected" => DduiClose::Disconnected,
                _ => DduiClose::Other,
            }),
            _ => return None,
        })
    }
}
