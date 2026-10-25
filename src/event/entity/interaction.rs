/**
A player attacks or interacts with an entity.

Handled by [`Event::on_entity_interact`](crate::prelude::Event::on_entity_interact).
*/
pub struct EntityInteractEvent {
    /// Name of the player.
    pub player_name: String,
    /// Runtime id of the entity.
    pub entity_id: u64,
    /// Type of the entity, such as `minecraft:zombie`.
    pub identifier: String,
    /// [`ATTACK`](Self::ATTACK) or [`INTERACT`](Self::INTERACT).
    pub action: u32,
}

impl EntityInteractEvent {
    /// [`action`](Self::action) of an attack.
    pub const ATTACK: u32 = 0;
    /// [`action`](Self::action) of a use, such as opening a villager's trades.
    pub const INTERACT: u32 = 1;

    /// Returns `true` when the player attacks the entity rather than using it.
    pub fn is_attack(&self) -> bool {
        self.action == Self::ATTACK
    }
}
