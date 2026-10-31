use crate::event::inventory::{
    AnvilUseEvent, BeaconChangeEvent, CraftItemEvent, EnchantItemEvent, GrindstoneUseEvent,
    SmithingUseEvent,
};
use crate::ffi::behaviour::{
    AnvilUseEventData, BeaconChangeEventData, CraftItemEventData, EnchantItemEventData,
    GrindstoneUseEventData, SmithingUseEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
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

impl CraftItemEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_craft_item<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &CraftItemEvent) -> V + Send + Sync + 'static,
    ) {
        register::<craft_item, V>(handler);
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

impl EnchantItemEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_enchant_item<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EnchantItemEvent) -> V + Send + Sync + 'static,
    ) {
        register::<enchant_item, V>(handler);
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

impl AnvilUseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_anvil_use<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &AnvilUseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<anvil_use, V>(handler);
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

impl GrindstoneUseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_grindstone_use<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &GrindstoneUseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<grindstone_use, V>(handler);
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

impl SmithingUseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_smithing_use<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &SmithingUseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<smithing_use, V>(handler);
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

impl BeaconChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_beacon_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BeaconChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<beacon_change, V>(handler);
    }
}
