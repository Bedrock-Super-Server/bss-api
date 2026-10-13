use crate::event::inventory::{
    AnvilUseEvent, BeaconChangeEvent, CraftItemEvent, EnchantItemEvent, GrindstoneUseEvent,
    SmithingUseEvent,
};
use crate::ffi::behaviour::{
    AnvilUseEventData, BeaconChangeEventData, CraftItemEventData, EnchantItemEventData,
    GrindstoneUseEventData, SmithingUseEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    craft_item,
    CraftItemEventData => CraftItemEvent,
    |raw: &CraftItemEventData| CraftItemEvent {
        player: ffi_string(raw.player),
        station: ffi_string(raw.station),
        recipe: raw.recipe,
        item: raw.item,
        count: raw.count,
    }
);

impl Cancellable for CraftItemEvent {}

impl Server {
    /**
    Calls `handler` when a player crafts an item.

    Returning `true` cancels the craft.

    See [`CraftItemEvent`].
    */
    pub fn on_craft_item(
        &self,
        handler: impl Fn(&Server, &CraftItemEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<craft_item>(handler);
    }
}

hook!(
    enchant_item,
    EnchantItemEventData => EnchantItemEvent,
    |raw: &EnchantItemEventData| EnchantItemEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        option: raw.option,
        levels: std::cell::Cell::new(raw.levels),
    },
    |safe: EnchantItemEvent, raw: &mut EnchantItemEventData| {
        raw.levels = safe.levels.get();
    }
);

impl Cancellable for EnchantItemEvent {}

impl Server {
    /**
    Calls `handler` when a player enchants an item at an enchanting table.

    Returning `true` cancels the enchantment.

    See [`EnchantItemEvent`].
    */
    pub fn on_enchant_item(
        &self,
        handler: impl Fn(&Server, &EnchantItemEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<enchant_item>(handler);
    }
}

hook!(
    anvil_use,
    AnvilUseEventData => AnvilUseEvent,
    |raw: &AnvilUseEventData| AnvilUseEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        name: ffi_string(raw.name),
        levels: std::cell::Cell::new(raw.levels),
    },
    |safe: AnvilUseEvent, raw: &mut AnvilUseEventData| {
        raw.levels = safe.levels.get();
    }
);

impl Cancellable for AnvilUseEvent {}

impl Server {
    /**
    Calls `handler` when a player takes the result out of an anvil.

    Returning `true` cancels the operation.

    See [`AnvilUseEvent`].
    */
    pub fn on_anvil_use(
        &self,
        handler: impl Fn(&Server, &AnvilUseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<anvil_use>(handler);
    }
}

hook!(
    grindstone_use,
    GrindstoneUseEventData => GrindstoneUseEvent,
    |raw: &GrindstoneUseEventData| GrindstoneUseEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        experience: std::cell::Cell::new(raw.experience),
    },
    |safe: GrindstoneUseEvent, raw: &mut GrindstoneUseEventData| {
        raw.experience = safe.experience.get();
    }
);

impl Cancellable for GrindstoneUseEvent {}

impl Server {
    /**
    Calls `handler` when a player takes the result out of a grindstone.

    Returning `true` cancels the operation.

    See [`GrindstoneUseEvent`].
    */
    pub fn on_grindstone_use(
        &self,
        handler: impl Fn(&Server, &GrindstoneUseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<grindstone_use>(handler);
    }
}

hook!(
    smithing_use,
    SmithingUseEventData => SmithingUseEvent,
    |raw: &SmithingUseEventData| SmithingUseEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        result: raw.result,
    }
);

impl Cancellable for SmithingUseEvent {}

impl Server {
    /**
    Calls `handler` when a player takes the result out of a smithing table.

    Returning `true` cancels the operation.

    See [`SmithingUseEvent`].
    */
    pub fn on_smithing_use(
        &self,
        handler: impl Fn(&Server, &SmithingUseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<smithing_use>(handler);
    }
}

hook!(
    beacon_change,
    BeaconChangeEventData => BeaconChangeEvent,
    |raw: &BeaconChangeEventData| BeaconChangeEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        primary: raw.primary,
        secondary: raw.secondary,
    }
);

impl Cancellable for BeaconChangeEvent {}

impl Server {
    /**
    Calls `handler` when a player changes the effects of a beacon.

    Returning `true` keeps the previous effects.

    See [`BeaconChangeEvent`].
    */
    pub fn on_beacon_change(
        &self,
        handler: impl Fn(&Server, &BeaconChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<beacon_change>(handler);
    }
}
