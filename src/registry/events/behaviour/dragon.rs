use crate::event::dragon::{DragonDeathEvent, DragonPhaseEvent, DragonSpawnEvent};
use crate::ffi::behaviour::{DragonDeathEventData, DragonPhaseEventData, DragonSpawnEventData};
use crate::registry::hook::{Cancellable, hook, register};
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

impl Server {
    /**
    Calls `handler` when an ender dragon is about to spawn in the End.

    Returning `true` prevents the dragon from spawning.

    See [`DragonSpawnEvent`].
    */
    pub fn on_dragon_spawn(
        &self,
        handler: impl Fn(&Server, &DragonSpawnEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<dragon_spawn>(handler);
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

impl Server {
    /**
    Calls `handler` when the ender dragon changes its behaviour phase.

    The value returned is ignored: this event cannot be cancelled.

    See [`DragonPhaseEvent`].
    */
    pub fn on_dragon_phase(
        &self,
        handler: impl Fn(&Server, &DragonPhaseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<dragon_phase>(handler);
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

impl Server {
    /**
    Calls `handler` when the ender dragon dies.

    The value returned is ignored: this event cannot be cancelled.

    See [`DragonDeathEvent`].
    */
    pub fn on_dragon_death(
        &self,
        handler: impl Fn(&Server, &DragonDeathEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<dragon_death>(handler);
    }
}
