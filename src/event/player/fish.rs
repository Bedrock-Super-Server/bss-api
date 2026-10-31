use std::cell::Cell;

/**
A player casts a fishing rod, or reels it in.

`event.cancel()` cancels the cast or the catch.

Handled by [`Event::on_fish`](crate::prelude::Event::on_fish) and [`Server::on_fish`](crate::server::Server::on_fish).
*/
pub struct FishEvent {
    /// Name of the player.
    pub player: String,
    /// What happens: `cast`, `catch`, `hook_entity` or `reel`.
    pub action: String,
    pub(crate) item: Cell<i16>,
    pub(crate) experience: Cell<u32>,
    /// Runtime id of the entity on the hook, or `0`.
    pub hooked: u64,
}

impl FishEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.player)
    }
}

impl FishEvent {
    /// Returns the numeric id of the item caught, or `0` for nothing.
    pub fn item(&self) -> i16 {
        self.item.get()
    }

    /// Sets the numeric id of the item caught, or `0` for nothing.
    pub fn set_item(&self, value: i16) {
        self.item.set(value);
    }

    /// Returns the experience the catch gives.
    pub fn experience(&self) -> u32 {
        self.experience.get()
    }

    /// Sets the experience the catch gives.
    pub fn set_experience(&self, value: u32) {
        self.experience.set(value);
    }
}
