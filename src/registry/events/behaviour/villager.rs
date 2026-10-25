use crate::event::villager::{VillagerLevelUpEvent, VillagerProfessionEvent, VillagerTradeEvent};
use crate::ffi::behaviour::{
    VillagerLevelUpEventData, VillagerProfessionEventData, VillagerTradeEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
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

impl Server {
    /**
    Calls `handler` when a villager takes a profession by claiming a workstation.

    Returning `true` keeps the villager unemployed.

    See [`VillagerProfessionEvent`].
    */
    pub fn on_villager_profession(
        &self,
        handler: impl Fn(&Server, &VillagerProfessionEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<villager_profession>(handler);
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

impl Server {
    /**
    Calls `handler` when a player trades with a villager or a wandering trader.

    Returning `true` cancels the trade.

    See [`VillagerTradeEvent`].
    */
    pub fn on_villager_trade(
        &self,
        handler: impl Fn(&Server, &VillagerTradeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<villager_trade>(handler);
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

impl Server {
    /**
    Calls `handler` when a villager reaches a new trading level.

    The value returned is ignored: this event cannot be cancelled.

    See [`VillagerLevelUpEvent`].
    */
    pub fn on_villager_level_up(
        &self,
        handler: impl Fn(&Server, &VillagerLevelUpEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<villager_level_up>(handler);
    }
}
