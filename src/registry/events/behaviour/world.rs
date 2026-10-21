use crate::event::world::{
    ExplosionBlocksEvent, ExplosionEvent, LightningStrikeEvent, RainChangeEvent, SpawnChangeEvent,
    ThunderChangeEvent, TickRateChangeEvent, WeatherEffectEvent, WorldLoadEvent,
};
use crate::ffi::behaviour::{
    ExplosionBlocksEventData, ExplosionEventData, LightningStrikeEventData, RainChangeEventData,
    SpawnChangeEventData, ThunderChangeEventData, TickRateChangeEventData, WeatherEffectEventData,
    WorldLoadEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    explosion,
    ExplosionEventData => ExplosionEvent,
    |raw: &ExplosionEventData| ExplosionEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        power: std::cell::Cell::new(raw.power),
        fire: std::cell::Cell::new(raw.fire),
        breaks_blocks: std::cell::Cell::new(raw.breaks_blocks),
        owner: raw.owner,
        world: raw.world,
    },
    |safe: ExplosionEvent, raw: &mut ExplosionEventData| {
        raw.power = safe.power.get();
        raw.fire = safe.fire.get();
        raw.breaks_blocks = safe.breaks_blocks.get();
    }
);

impl Cancellable for ExplosionEvent {}

impl Server {
    /**
    Calls `handler` when an explosion is about to happen.

    Returning `true` cancels the explosion.

    See [`ExplosionEvent`].
    */
    pub fn on_explosion(
        &self,
        handler: impl Fn(&Server, &ExplosionEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<explosion>(handler);
    }
}

hook!(
    explosion_blocks,
    ExplosionBlocksEventData => ExplosionBlocksEvent,
    |raw: &ExplosionBlocksEventData| ExplosionBlocksEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        count: raw.count,
        drop_items: std::cell::Cell::new(raw.drop_items),
        world: raw.world,
    },
    |safe: ExplosionBlocksEvent, raw: &mut ExplosionBlocksEventData| {
        raw.drop_items = safe.drop_items.get();
    }
);

impl Cancellable for ExplosionBlocksEvent {}

impl Server {
    /**
    Calls `handler` when an explosion is about to destroy blocks.

    Returning `true` keeps every block in place.

    See [`ExplosionBlocksEvent`].
    */
    pub fn on_explosion_blocks(
        &self,
        handler: impl Fn(&Server, &ExplosionBlocksEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<explosion_blocks>(handler);
    }
}

hook!(
    lightning_strike,
    LightningStrikeEventData => LightningStrikeEvent,
    |raw: &LightningStrikeEventData| LightningStrikeEvent {
        x: std::cell::Cell::new(raw.x),
        y: std::cell::Cell::new(raw.y),
        z: std::cell::Cell::new(raw.z),
        cause: ffi_string(raw.cause),
        fire: std::cell::Cell::new(raw.fire),
        owner: raw.owner,
        world: raw.world,
    },
    |safe: LightningStrikeEvent, raw: &mut LightningStrikeEventData| {
        raw.x = safe.x.get();
        raw.y = safe.y.get();
        raw.z = safe.z.get();
        raw.fire = safe.fire.get();
    }
);

impl Cancellable for LightningStrikeEvent {}

impl Server {
    /**
    Calls `handler` when lightning is about to strike.

    Returning `true` cancels the strike.

    See [`LightningStrikeEvent`].
    */
    pub fn on_lightning_strike(
        &self,
        handler: impl Fn(&Server, &LightningStrikeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<lightning_strike>(handler);
    }
}

hook!(
    rain_change,
    RainChangeEventData => RainChangeEvent,
    |raw: &RainChangeEventData| RainChangeEvent {
        raining: raw.raining,
        duration: std::cell::Cell::new(raw.duration),
        world: raw.world,
    },
    |safe: RainChangeEvent, raw: &mut RainChangeEventData| {
        raw.duration = safe.duration.get();
    }
);

impl Cancellable for RainChangeEvent {}

impl Server {
    /**
    Calls `handler` when rain starts or stops.

    Returning `true` keeps the current weather.

    See [`RainChangeEvent`].
    */
    pub fn on_rain_change(
        &self,
        handler: impl Fn(&Server, &RainChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<rain_change>(handler);
    }
}

hook!(
    thunder_change,
    ThunderChangeEventData => ThunderChangeEvent,
    |raw: &ThunderChangeEventData| ThunderChangeEvent {
        thundering: raw.thundering,
        duration: std::cell::Cell::new(raw.duration),
        world: raw.world,
    },
    |safe: ThunderChangeEvent, raw: &mut ThunderChangeEventData| {
        raw.duration = safe.duration.get();
    }
);

impl Cancellable for ThunderChangeEvent {}

impl Server {
    /**
    Calls `handler` when a thunderstorm starts or stops.

    Returning `true` keeps the current weather.

    See [`ThunderChangeEvent`].
    */
    pub fn on_thunder_change(
        &self,
        handler: impl Fn(&Server, &ThunderChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<thunder_change>(handler);
    }
}

hook!(
    weather_effect,
    WeatherEffectEventData => WeatherEffectEvent,
    |raw: &WeatherEffectEventData| WeatherEffectEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        new_block: std::cell::Cell::new(raw.new_block),
        cause: ffi_string(raw.cause),
        world: raw.world,
    },
    |safe: WeatherEffectEvent, raw: &mut WeatherEffectEventData| {
        raw.new_block = safe.new_block.get();
    }
);

impl Cancellable for WeatherEffectEvent {}

impl Server {
    /**
    Calls `handler` when the weather changes a block: snow piles up, water freezes, rain puts a fire out...

    Returning `true` leaves the block as it is.

    See [`WeatherEffectEvent`].
    */
    pub fn on_weather_effect(
        &self,
        handler: impl Fn(&Server, &WeatherEffectEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<weather_effect>(handler);
    }
}

hook!(
    spawn_change,
    SpawnChangeEventData => SpawnChangeEvent,
    |raw: &SpawnChangeEventData| SpawnChangeEvent {
        x: std::cell::Cell::new(raw.x),
        y: std::cell::Cell::new(raw.y),
        z: std::cell::Cell::new(raw.z),
        world: raw.world,
    },
    |safe: SpawnChangeEvent, raw: &mut SpawnChangeEventData| {
        raw.x = safe.x.get();
        raw.y = safe.y.get();
        raw.z = safe.z.get();
    }
);

impl Cancellable for SpawnChangeEvent {}

impl Server {
    /**
    Calls `handler` when the spawn point of a world is about to move, with `/setworldspawn` or a plugin. Set another position to move it there instead.

    Returning `true` from a handler keeps the spawn point where it is.

    See [`SpawnChangeEvent`].
    */
    pub fn on_spawn_change(
        &self,
        handler: impl Fn(&Server, &SpawnChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<spawn_change>(handler);
    }
}

hook!(
    world_load,
    WorldLoadEventData => WorldLoadEvent,
    |raw: &WorldLoadEventData| WorldLoadEvent {
        world: raw.world,
        name: ffi_string(raw.name),
    }
);

impl Cancellable for WorldLoadEvent {}

impl Server {
    /**
    Calls `handler` when a world finishes loading and is ready for players and plugins.

    Returning `true` changes nothing.

    See [`WorldLoadEvent`].
    */
    pub fn on_world_load(
        &self,
        handler: impl Fn(&Server, &WorldLoadEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<world_load>(handler);
    }
}

hook!(
    tick_rate_change,
    TickRateChangeEventData => TickRateChangeEvent,
    |raw: &TickRateChangeEventData| TickRateChangeEvent {
        rate: std::cell::Cell::new(raw.rate),
        frozen: std::cell::Cell::new(raw.frozen),
    },
    |safe: TickRateChangeEvent, raw: &mut TickRateChangeEventData| {
        raw.rate = safe.rate.get();
        raw.frozen = safe.frozen.get();
    }
);

impl Cancellable for TickRateChangeEvent {}

impl Server {
    /**
    Calls `handler` when the tick rate of the server is about to change, through `/tick` or a plugin.

    Returning `true` keeps the current rate.

    See [`TickRateChangeEvent`].
    */
    pub fn on_tick_rate_change(
        &self,
        handler: impl Fn(&Server, &TickRateChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<tick_rate_change>(handler);
    }
}
