use crate::event::world::{
    ExplosionBlocksEvent, ExplosionEvent, LightningStrikeEvent, RainChangeEvent, SpawnChangeEvent,
    ThunderChangeEvent, TickRateChangeEvent, WeatherEffectEvent, WorldLoadEvent,
};
use crate::ffi::behaviour::{
    ExplosionBlocksEventData, ExplosionEventData, LightningStrikeEventData, RainChangeEventData,
    SpawnChangeEventData, ThunderChangeEventData, TickRateChangeEventData, WeatherEffectEventData,
    WorldLoadEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
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

impl ExplosionEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_explosion<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ExplosionEvent) -> V + Send + Sync + 'static,
    ) {
        register::<explosion, V>(handler);
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

impl ExplosionBlocksEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_explosion_blocks<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ExplosionBlocksEvent) -> V + Send + Sync + 'static,
    ) {
        register::<explosion_blocks, V>(handler);
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

impl LightningStrikeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_lightning_strike<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &LightningStrikeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<lightning_strike, V>(handler);
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

impl RainChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_rain_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &RainChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<rain_change, V>(handler);
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

impl ThunderChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_thunder_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ThunderChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<thunder_change, V>(handler);
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

impl WeatherEffectEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_weather_effect<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &WeatherEffectEvent) -> V + Send + Sync + 'static,
    ) {
        register::<weather_effect, V>(handler);
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

impl SpawnChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_spawn_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &SpawnChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<spawn_change, V>(handler);
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

impl WorldLoadEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_world_load<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &WorldLoadEvent) -> V + Send + Sync + 'static,
    ) {
        register::<world_load, V>(handler);
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

impl TickRateChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_tick_rate_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &TickRateChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<tick_rate_change, V>(handler);
    }
}
