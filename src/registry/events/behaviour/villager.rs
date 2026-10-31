use crate::event::villager::{VillagerLevelUpEvent, VillagerProfessionEvent, VillagerTradeEvent};
use crate::ffi::behaviour::{
    VillagerLevelUpEventData, VillagerProfessionEventData, VillagerTradeEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    villager_profession,
    VillagerProfessionEventData => VillagerProfessionEvent,
    |raw: &VillagerProfessionEventData| VillagerProfessionEvent {
        entity: raw.entity,
        profession: ffi_string(raw.profession),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        world: raw.world,
    }
);

impl Cancellable for VillagerProfessionEvent {}

impl VillagerProfessionEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_villager_profession<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &VillagerProfessionEvent) -> V + Send + Sync + 'static,
    ) {
        register::<villager_profession, V>(handler);
    }
}

hook!(
    villager_trade,
    VillagerTradeEventData => VillagerTradeEvent,
    |raw: &VillagerTradeEventData| VillagerTradeEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        offer: raw.offer,
        times: raw.times,
        item: raw.item,
        experience: std::cell::Cell::new(raw.experience),
    },
    |safe: VillagerTradeEvent, raw: &mut VillagerTradeEventData| {
        raw.experience = safe.experience.get();
    }
);

impl Cancellable for VillagerTradeEvent {}

impl VillagerTradeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_villager_trade<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &VillagerTradeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<villager_trade, V>(handler);
    }
}

hook!(
    villager_level_up,
    VillagerLevelUpEventData => VillagerLevelUpEvent,
    |raw: &VillagerLevelUpEventData| VillagerLevelUpEvent {
        entity: raw.entity,
        level: raw.level,
        profession: ffi_string(raw.profession),
        world: raw.world,
    }
);

impl Cancellable for VillagerLevelUpEvent {}

impl VillagerLevelUpEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_villager_level_up<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &VillagerLevelUpEvent) -> V + Send + Sync + 'static,
    ) {
        register::<villager_level_up, V>(handler);
    }
}
