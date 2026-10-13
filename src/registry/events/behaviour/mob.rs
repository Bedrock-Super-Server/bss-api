use crate::event::mob::{
    MobAttackEvent, MobBreedEvent, MobDespawnEvent, MobInteractEvent, MobLeashEvent, MobSpawnEvent,
    MobTameEvent, MobTargetEvent,
};
use crate::ffi::behaviour::{
    MobAttackEventData, MobBreedEventData, MobDespawnEventData, MobInteractEventData,
    MobLeashEventData, MobSpawnEventData, MobTameEventData, MobTargetEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    mob_spawn,
    MobSpawnEventData => MobSpawnEvent,
    |raw: &MobSpawnEventData| MobSpawnEvent {
        identifier: ffi_string(raw.identifier),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        cause: ffi_string(raw.cause),
        world: raw.world,
    }
);

impl Cancellable for MobSpawnEvent {}

impl Server {
    /**
    Calls `handler` when a mob is about to spawn.

    Returning `true` prevents the spawn.

    See [`MobSpawnEvent`].
    */
    pub fn on_mob_spawn(
        &self,
        handler: impl Fn(&Server, &MobSpawnEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_spawn>(handler);
    }
}

hook!(
    mob_despawn,
    MobDespawnEventData => MobDespawnEvent,
    |raw: &MobDespawnEventData| MobDespawnEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        world: raw.world,
    }
);

impl Cancellable for MobDespawnEvent {}

impl Server {
    /**
    Calls `handler` when a mob is about to despawn because no player is close enough.

    Returning `true` keeps the mob.

    See [`MobDespawnEvent`].
    */
    pub fn on_mob_despawn(
        &self,
        handler: impl Fn(&Server, &MobDespawnEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_despawn>(handler);
    }
}

hook!(
    mob_target,
    MobTargetEventData => MobTargetEvent,
    |raw: &MobTargetEventData| MobTargetEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        target: std::cell::Cell::new(raw.target),
        world: raw.world,
    },
    |safe: MobTargetEvent, raw: &mut MobTargetEventData| {
        raw.target = safe.target.get();
    }
);

impl Cancellable for MobTargetEvent {}

impl Server {
    /**
    Calls `handler` when a mob picks a new target.

    Returning `true` leaves the mob without a target.

    See [`MobTargetEvent`].
    */
    pub fn on_mob_target(
        &self,
        handler: impl Fn(&Server, &MobTargetEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_target>(handler);
    }
}

hook!(
    mob_attack,
    MobAttackEventData => MobAttackEvent,
    |raw: &MobAttackEventData| MobAttackEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        target: raw.target,
        damage: std::cell::Cell::new(raw.damage),
        world: raw.world,
    },
    |safe: MobAttackEvent, raw: &mut MobAttackEventData| {
        raw.damage = safe.damage.get();
    }
);

impl Cancellable for MobAttackEvent {}

impl Server {
    /**
    Calls `handler` when a mob attacks its target.

    Returning `true` cancels the attack.

    See [`MobAttackEvent`].
    */
    pub fn on_mob_attack(
        &self,
        handler: impl Fn(&Server, &MobAttackEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_attack>(handler);
    }
}

hook!(
    mob_breed,
    MobBreedEventData => MobBreedEvent,
    |raw: &MobBreedEventData| MobBreedEvent {
        parent_a: raw.parent_a,
        parent_b: raw.parent_b,
        identifier: ffi_string(raw.identifier),
        experience: std::cell::Cell::new(raw.experience),
        world: raw.world,
    },
    |safe: MobBreedEvent, raw: &mut MobBreedEventData| {
        raw.experience = safe.experience.get();
    }
);

impl Cancellable for MobBreedEvent {}

impl Server {
    /**
    Calls `handler` when two mobs breed and a baby is about to be born.

    Returning `true` prevents the birth.

    See [`MobBreedEvent`].
    */
    pub fn on_mob_breed(
        &self,
        handler: impl Fn(&Server, &MobBreedEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_breed>(handler);
    }
}

hook!(
    mob_interact,
    MobInteractEventData => MobInteractEvent,
    |raw: &MobInteractEventData| MobInteractEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        item: raw.item,
        action: ffi_string(raw.action),
    }
);

impl Cancellable for MobInteractEvent {}

impl Server {
    /**
    Calls `handler` when a player uses an item on a mob: feeds, tames, shears, milks, names...

    Returning `true` cancels the interaction.

    See [`MobInteractEvent`].
    */
    pub fn on_mob_interact(
        &self,
        handler: impl Fn(&Server, &MobInteractEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_interact>(handler);
    }
}

hook!(
    mob_tame,
    MobTameEventData => MobTameEvent,
    |raw: &MobTameEventData| MobTameEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        world: raw.world,
    }
);

impl Cancellable for MobTameEvent {}

impl Server {
    /**
    Calls `handler` when a player tames a mob.

    Returning `true` makes this taming attempt fail.

    See [`MobTameEvent`].
    */
    pub fn on_mob_tame(
        &self,
        handler: impl Fn(&Server, &MobTameEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_tame>(handler);
    }
}

hook!(
    mob_leash,
    MobLeashEventData => MobLeashEvent,
    |raw: &MobLeashEventData| MobLeashEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        leashed: raw.leashed,
        fence: raw.fence,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        world: raw.world,
    }
);

impl Cancellable for MobLeashEvent {}

impl Server {
    /**
    Calls `handler` when a mob is leashed or unleashed.

    Returning `true` leaves the leash as it was.

    See [`MobLeashEvent`].
    */
    pub fn on_mob_leash(
        &self,
        handler: impl Fn(&Server, &MobLeashEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<mob_leash>(handler);
    }
}
