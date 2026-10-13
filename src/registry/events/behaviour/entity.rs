use crate::event::entity::{
    ArmorStandChangeEvent, EntityCombustEvent, EntityHurtEvent, EntityKilledEvent,
    EntityPlaceEvent, EntityTouchEvent, ItemDespawnEvent, ItemSpawnEvent,
};
use crate::ffi::behaviour::{
    ArmorStandChangeEventData, EntityCombustEventData, EntityHurtEventData, EntityKilledEventData,
    EntityPlaceEventData, EntityTouchEventData, ItemDespawnEventData, ItemSpawnEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
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

impl Server {
    /**
    Calls `handler` when a player places an entity from an item: an armor stand, a boat, a minecart, an end crystal...

    Returning `true` cancels the placement and keeps the item.

    See [`EntityPlaceEvent`].
    */
    pub fn on_entity_place(
        &self,
        handler: impl Fn(&Server, &EntityPlaceEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<entity_place>(handler);
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

impl Server {
    /**
    Calls `handler` when a player touches or hits an entity that has no behaviour of its own, such as a plugin entity.

    Returning `true` ignores the touch.

    See [`EntityTouchEvent`].
    */
    pub fn on_entity_touch(
        &self,
        handler: impl Fn(&Server, &EntityTouchEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<entity_touch>(handler);
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

impl Server {
    /**
    Calls `handler` when an entity other than a player takes damage from any source.

    Returning `true` cancels the damage.

    See [`EntityHurtEvent`].
    */
    pub fn on_entity_hurt(
        &self,
        handler: impl Fn(&Server, &EntityHurtEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<entity_hurt>(handler);
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

impl Server {
    /**
    Calls `handler` when an entity other than a player is killed.

    Returning `true` keeps the entity alive, with one health point.

    See [`EntityKilledEvent`].
    */
    pub fn on_entity_killed(
        &self,
        handler: impl Fn(&Server, &EntityKilledEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<entity_killed>(handler);
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

impl Server {
    /**
    Calls `handler` when a player puts an item on an armor stand or takes one off.

    Returning `true` leaves the armor stand as it was.

    See [`ArmorStandChangeEvent`].
    */
    pub fn on_armor_stand_change(
        &self,
        handler: impl Fn(&Server, &ArmorStandChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<armor_stand_change>(handler);
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

impl Server {
    /**
    Calls `handler` when an entity other than a player catches fire.

    Returning `true` keeps the entity from burning.

    See [`EntityCombustEvent`].
    */
    pub fn on_entity_combust(
        &self,
        handler: impl Fn(&Server, &EntityCombustEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<entity_combust>(handler);
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

impl Server {
    /**
    Calls `handler` when an item entity appears in the world: a drop, a broken block, a dispenser throw...

    Returning `true` prevents the item from appearing.

    See [`ItemSpawnEvent`].
    */
    pub fn on_item_spawn(
        &self,
        handler: impl Fn(&Server, &ItemSpawnEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_spawn>(handler);
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

impl Server {
    /**
    Calls `handler` when a dropped item disappears after lying on the ground for five minutes.

    The value returned is ignored: this event cannot be cancelled.

    See [`ItemDespawnEvent`].
    */
    pub fn on_item_despawn(
        &self,
        handler: impl Fn(&Server, &ItemDespawnEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_despawn>(handler);
    }
}
