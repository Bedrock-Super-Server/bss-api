/// What a [`ToggleEvent`] reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleKind {
    /// The player sneaks.
    Sneak,
    /// The player sprints.
    Sprint,
    /// The player swims.
    Swim,
    /// The player glides with elytra.
    Glide,
}

impl ToggleKind {
    pub(crate) fn from_raw(kind: u32) -> Self {
        match kind {
            1 => ToggleKind::Sprint,
            2 => ToggleKind::Swim,
            3 => ToggleKind::Glide,
            _ => ToggleKind::Sneak,
        }
    }
}

/**
A player starts or stops sneaking, sprinting, swimming or gliding.

Handled by [`Event::on_toggle`](crate::prelude::Event::on_toggle).
*/
pub struct ToggleEvent {
    /// Name of the player.
    pub name: String,
    /// What was toggled.
    pub kind: ToggleKind,
    /// `true` when it starts, `false` when it stops.
    pub active: bool,
}

impl ToggleEvent {
    /// The player, or `None` when they have already left the server.
    pub fn player(&self) -> Option<crate::player::Player> {
        crate::registry::server().player(&self.name)
    }
}
