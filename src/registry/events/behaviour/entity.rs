use crate::event::entity::{
    ArmorStandChangeEvent, EntityCombustEvent, EntityHurtEvent, EntityKilledEvent,
    EntityPlaceEvent, EntityTouchEvent, ItemDespawnEvent, ItemSpawnEvent,
};
use crate::ffi::behaviour::{
    ArmorStandChangeEventData, EntityCombustEventData, EntityHurtEventData, EntityKilledEventData,
    EntityPlaceEventData, EntityTouchEventData, ItemDespawnEventData, ItemSpawnEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    entity_place,
    EntityPlaceEventData => EntityPlaceEvent,
    |raw: &EntityPlaceEventData| EntityPlaceEvent {
        player: ffi_string(raw.player),
        identifier: ffi_string(raw.identifier),
        x: raw.x,
        y: raw.y,
        z: raw.z,
    }
);

impl Cancellable for EntityPlaceEvent {}

impl EntityPlaceEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_entity_place<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EntityPlaceEvent) -> V + Send + Sync + 'static,
    ) {
        register::<entity_place, V>(handler);
    }
}

hook!(
    entity_touch,
    EntityTouchEventData => EntityTouchEvent,
    |raw: &EntityTouchEventData| EntityTouchEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        attack: raw.attack,
    }
);

impl Cancellable for EntityTouchEvent {}

impl EntityTouchEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_entity_touch<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EntityTouchEvent) -> V + Send + Sync + 'static,
    ) {
        register::<entity_touch, V>(handler);
    }
}

hook!(
    entity_hurt,
    EntityHurtEventData => EntityHurtEvent,
    |raw: &EntityHurtEventData| EntityHurtEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        damage: std::cell::Cell::new(raw.damage),
        attacker: raw.attacker,
    },
    |safe: EntityHurtEvent, raw: &mut EntityHurtEventData| {
        raw.damage = safe.damage.get();
    }
);

impl Cancellable for EntityHurtEvent {}

impl EntityHurtEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_entity_hurt<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EntityHurtEvent) -> V + Send + Sync + 'static,
    ) {
        register::<entity_hurt, V>(handler);
    }
}

hook!(
    entity_killed,
    EntityKilledEventData => EntityKilledEvent,
    |raw: &EntityKilledEventData| EntityKilledEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        killer: raw.killer,
        experience: std::cell::Cell::new(raw.experience),
        drops: std::cell::Cell::new(raw.drops),
    },
    |safe: EntityKilledEvent, raw: &mut EntityKilledEventData| {
        raw.experience = safe.experience.get();
        raw.drops = safe.drops.get();
    }
);

impl Cancellable for EntityKilledEvent {}

impl EntityKilledEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_entity_killed<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EntityKilledEvent) -> V + Send + Sync + 'static,
    ) {
        register::<entity_killed, V>(handler);
    }
}

hook!(
    armor_stand_change,
    ArmorStandChangeEventData => ArmorStandChangeEvent,
    |raw: &ArmorStandChangeEventData| ArmorStandChangeEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        slot: raw.slot,
        item: raw.item,
    }
);

impl Cancellable for ArmorStandChangeEvent {}

impl ArmorStandChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_armor_stand_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ArmorStandChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<armor_stand_change, V>(handler);
    }
}

hook!(
    entity_combust,
    EntityCombustEventData => EntityCombustEvent,
    |raw: &EntityCombustEventData| EntityCombustEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        ticks: std::cell::Cell::new(raw.ticks),
        cause: ffi_string(raw.cause),
        world: raw.world,
    },
    |safe: EntityCombustEvent, raw: &mut EntityCombustEventData| {
        raw.ticks = safe.ticks.get();
    }
);

impl Cancellable for EntityCombustEvent {}

impl EntityCombustEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_entity_combust<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EntityCombustEvent) -> V + Send + Sync + 'static,
    ) {
        register::<entity_combust, V>(handler);
    }
}

hook!(
    item_spawn,
    ItemSpawnEventData => ItemSpawnEvent,
    |raw: &ItemSpawnEventData| ItemSpawnEvent {
        entity: raw.entity,
        item: raw.item,
        count: raw.count,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        world: raw.world,
    }
);

impl Cancellable for ItemSpawnEvent {}

impl ItemSpawnEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_spawn<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemSpawnEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_spawn, V>(handler);
    }
}

hook!(
    item_despawn,
    ItemDespawnEventData => ItemDespawnEvent,
    |raw: &ItemDespawnEventData| ItemDespawnEvent {
        entity: raw.entity,
        item: raw.item,
        count: raw.count,
    }
);

impl Cancellable for ItemDespawnEvent {}

impl ItemDespawnEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_despawn<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemDespawnEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_despawn, V>(handler);
    }
}
