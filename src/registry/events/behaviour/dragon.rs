use crate::event::dragon::{DragonDeathEvent, DragonPhaseEvent, DragonSpawnEvent};
use crate::ffi::behaviour::{DragonDeathEventData, DragonPhaseEventData, DragonSpawnEventData};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    dragon_spawn,
    DragonSpawnEventData => DragonSpawnEvent,
    |raw: &DragonSpawnEventData| DragonSpawnEvent {
        entity: raw.entity,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        respawn: raw.respawn,
        world: raw.world,
    }
);

impl Cancellable for DragonSpawnEvent {}

impl DragonSpawnEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_dragon_spawn<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DragonSpawnEvent) -> V + Send + Sync + 'static,
    ) {
        register::<dragon_spawn, V>(handler);
    }
}

hook!(
    dragon_phase,
    DragonPhaseEventData => DragonPhaseEvent,
    |raw: &DragonPhaseEventData| DragonPhaseEvent {
        entity: raw.entity,
        phase: ffi_string(raw.phase),
        world: raw.world,
    }
);

impl Cancellable for DragonPhaseEvent {}

impl DragonPhaseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_dragon_phase<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DragonPhaseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<dragon_phase, V>(handler);
    }
}

hook!(
    dragon_death,
    DragonDeathEventData => DragonDeathEvent,
    |raw: &DragonDeathEventData| DragonDeathEvent {
        entity: raw.entity,
        killer: ffi_string(raw.killer),
        first: raw.first,
        experience: std::cell::Cell::new(raw.experience),
        egg: std::cell::Cell::new(raw.egg),
        world: raw.world,
    },
    |safe: DragonDeathEvent, raw: &mut DragonDeathEventData| {
        raw.experience = safe.experience.get();
        raw.egg = safe.egg.get();
    }
);

impl Cancellable for DragonDeathEvent {}

impl DragonDeathEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_dragon_death<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DragonDeathEvent) -> V + Send + Sync + 'static,
    ) {
        register::<dragon_death, V>(handler);
    }
}
