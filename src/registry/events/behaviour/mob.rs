use crate::event::mob::{
    MobAttackEvent, MobBreedEvent, MobDespawnEvent, MobInteractEvent, MobLeashEvent, MobSpawnEvent,
    MobTameEvent, MobTargetEvent,
};
use crate::ffi::behaviour::{
    MobAttackEventData, MobBreedEventData, MobDespawnEventData, MobInteractEventData,
    MobLeashEventData, MobSpawnEventData, MobTameEventData, MobTargetEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
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

impl MobSpawnEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_spawn<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobSpawnEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_spawn, V>(handler);
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

impl MobDespawnEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_despawn<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobDespawnEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_despawn, V>(handler);
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

impl MobTargetEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_target<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobTargetEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_target, V>(handler);
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

impl MobAttackEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_attack<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobAttackEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_attack, V>(handler);
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

impl MobBreedEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_breed<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobBreedEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_breed, V>(handler);
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

impl MobInteractEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_interact<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobInteractEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_interact, V>(handler);
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

impl MobTameEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_tame<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobTameEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_tame, V>(handler);
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

impl MobLeashEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_mob_leash<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MobLeashEvent) -> V + Send + Sync + 'static,
    ) {
        register::<mob_leash, V>(handler);
    }
}
