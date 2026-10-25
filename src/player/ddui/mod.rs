//! Data-driven screens: windows that stay open while a plugin and a player change what they show.
//!
//! Build a [`DduiScreen`] and show it with [`Player::send_ddui`](crate::player::Player::send_ddui). It calls a handler with each
//! [`DduiEvent`], and returns a [`DduiHandle`] that changes the screen while it is open.

mod element;
mod event;
mod handle;
mod handlers;
mod screen;
mod send;

pub use element::{Element, ElementKind};
pub use event::{DduiClose, DduiEvent};
pub use handle::DduiHandle;
pub use screen::DduiScreen;
