use std::cell::Cell;

/**
A player attacks an entity in melee.

Returning `true` from a handler cancels the hit.

Handled by [`Event::on_entity_damage`](crate::prelude::Event::on_entity_damage).
*/
pub struct EntityDamageEvent {
    /// Name of the attacking player.
    pub attacker: String,
    /// Runtime id of the attacking player.
    pub attacker_entity_id: u64,
    /// Runtime id of the entity hit.
    pub target_entity_id: u64,
    /// Numeric id of the item used, or `0` for a bare hand.
    pub weapon_item_id: i16,
    pub(crate) damage: Cell<f32>,
    pub(crate) knockback_force: Cell<f32>,
    pub(crate) knockback_vertical: Cell<f32>,
    pub(crate) attack_cooldown: Cell<u64>,
}

impl EntityDamageEvent {
    /// Returns the damage dealt, in half-hearts.
    pub fn damage(&self) -> f32 {
        self.damage.get()
    }

    /// Sets the damage dealt, in half-hearts.
    pub fn set_damage(&self, damage: f32) {
        self.damage.set(damage);
    }

    /// Returns the horizontal knockback given to the target.
    pub fn knockback_force(&self) -> f32 {
        self.knockback_force.get()
    }

    /// Sets the horizontal knockback given to the target.
    pub fn set_knockback_force(&self, force: f32) {
        self.knockback_force.set(force);
    }

    /// Returns the vertical knockback given to the target.
    pub fn knockback_vertical(&self) -> f32 {
        self.knockback_vertical.get()
    }

    /// Sets the vertical knockback given to the target.
    pub fn set_knockback_vertical(&self, vertical: f32) {
        self.knockback_vertical.set(vertical);
    }

    /// Returns the time the attacker must wait before the next hit counts, in milliseconds.
    pub fn attack_cooldown(&self) -> u64 {
        self.attack_cooldown.get()
    }

    /// Sets the time the attacker must wait before the next hit counts, in milliseconds.
    pub fn set_attack_cooldown(&self, cooldown_ms: u64) {
        self.attack_cooldown.set(cooldown_ms);
    }
}
