use std::cell::Cell;

use crate::event::entity::{
    EntityDamageEvent, EntityDeathEvent, EntityDespawnEvent, EntityInteractEvent, EntitySpawnEvent,
};
use crate::ffi::events::*;
use crate::registry::macros::{event_hook, event_hook_cancel};
use crate::server::{Server, ffi_string};

event_hook_cancel!(
    on_entity_damage,
    ENTITY_DAMAGE,
    damage,
    EntityDamageByEntityEventData,
    EntityDamageEvent,
    |e: &EntityDamageByEntityEventData| {
        EntityDamageEvent {
            attacker: ffi_string(e.attacker),
            attacker_entity_id: e.attacker_entity_id,
            target_entity_id: e.target_entity_id,
            weapon_item_id: e.weapon_item_id,
            damage: Cell::new(e.damage),
            knockback_force: Cell::new(e.knockback_force),
            knockback_vertical: Cell::new(e.knockback_vertical),
            attack_cooldown: Cell::new(e.attack_cooldown),
        }
    },
    [damage, knockback_force, knockback_vertical, attack_cooldown]
);

event_hook!(
    on_entity_interact,
    ENTITY_INTERACT,
    entity_interact,
    EntityInteractEventData,
    EntityInteractEvent,
    |e: &EntityInteractEventData| {
        EntityInteractEvent {
            player_name: ffi_string(e.player_name),
            entity_id: e.entity_id,
            identifier: ffi_string(e.identifier),
            action: e.action,
        }
    }
);

event_hook_cancel!(
    on_entity_death,
    ENTITY_DEATH,
    entity_death,
    EntityDeathEventData,
    EntityDeathEvent,
    |e: &EntityDeathEventData| {
        EntityDeathEvent {
            entity_id: e.entity_id,
            identifier: ffi_string(e.identifier),
        }
    }
);

event_hook!(
    on_entity_spawn,
    ENTITY_SPAWN,
    entity_spawn,
    EntitySpawnEventData,
    EntitySpawnEvent,
    |e: &EntitySpawnEventData| EntitySpawnEvent {
        entity_id: e.entity_id,
        identifier: ffi_string(e.identifier),
        x: e.x,
        y: e.y,
        z: e.z,
        world_id: e.world_id,
    }
);

event_hook!(
    on_entity_despawn,
    ENTITY_DESPAWN,
    entity_despawn,
    EntityDespawnEventData,
    EntityDespawnEvent,
    |e: &EntityDespawnEventData| EntityDespawnEvent {
        entity_id: e.entity_id,
        identifier: ffi_string(e.identifier),
    }
);
