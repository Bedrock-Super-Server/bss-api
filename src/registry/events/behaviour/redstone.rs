use crate::event::redstone::{
    BlockDispenseEvent, CrafterCraftEvent, DetectorRailEvent, DispenserEquipEvent,
    HopperPickupEvent, ItemTransferEvent, MinecartActivateEvent, MinecartPickupEvent,
    MinecartTransferEvent, PistonMoveEvent, PistonPushEvent, PressurePlateEvent,
    RedstonePowerEvent, SculkVibrationEvent, TripwireEvent,
};
use crate::ffi::behaviour::{
    BlockDispenseEventData, CrafterCraftEventData, DetectorRailEventData, DispenserEquipEventData,
    HopperPickupEventData, ItemTransferEventData, MinecartActivateEventData,
    MinecartPickupEventData, MinecartTransferEventData, PistonMoveEventData, PistonPushEventData,
    PressurePlateEventData, RedstonePowerEventData, SculkVibrationEventData, TripwireEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    redstone_power,
    RedstonePowerEventData => RedstonePowerEvent,
    |raw: &RedstonePowerEventData| RedstonePowerEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        old_power: raw.old_power,
        new_power: raw.new_power,
        world: raw.world,
    }
);

impl Cancellable for RedstonePowerEvent {}

impl RedstonePowerEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_redstone_power<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &RedstonePowerEvent) -> V + Send + Sync + 'static,
    ) {
        register::<redstone_power, V>(handler);
    }
}

hook!(
    piston_move,
    PistonMoveEventData => PistonMoveEvent,
    |raw: &PistonMoveEventData| PistonMoveEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        extending: raw.extending,
        count: raw.count,
        world: raw.world,
    }
);

impl Cancellable for PistonMoveEvent {}

impl PistonMoveEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_piston_move<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PistonMoveEvent) -> V + Send + Sync + 'static,
    ) {
        register::<piston_move, V>(handler);
    }
}

hook!(
    piston_push,
    PistonPushEventData => PistonPushEvent,
    |raw: &PistonPushEventData| PistonPushEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        entity: raw.entity,
        player: ffi_string(raw.player),
        motion_x: std::cell::Cell::new(raw.motion_x),
        motion_y: std::cell::Cell::new(raw.motion_y),
        motion_z: std::cell::Cell::new(raw.motion_z),
        world: raw.world,
    },
    |safe: PistonPushEvent, raw: &mut PistonPushEventData| {
        raw.motion_x = safe.motion_x.get();
        raw.motion_y = safe.motion_y.get();
        raw.motion_z = safe.motion_z.get();
    }
);

impl Cancellable for PistonPushEvent {}

impl PistonPushEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_piston_push<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PistonPushEvent) -> V + Send + Sync + 'static,
    ) {
        register::<piston_push, V>(handler);
    }
}

hook!(
    minecart_activate,
    MinecartActivateEventData => MinecartActivateEvent,
    |raw: &MinecartActivateEventData| MinecartActivateEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        rider: raw.rider,
        world: raw.world,
    }
);

impl Cancellable for MinecartActivateEvent {}

impl MinecartActivateEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_minecart_activate<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MinecartActivateEvent) -> V + Send + Sync + 'static,
    ) {
        register::<minecart_activate, V>(handler);
    }
}

hook!(
    minecart_transfer,
    MinecartTransferEventData => MinecartTransferEvent,
    |raw: &MinecartTransferEventData| MinecartTransferEvent {
        entity: raw.entity,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        item: raw.item,
        into_cart: raw.into_cart,
        world: raw.world,
    }
);

impl Cancellable for MinecartTransferEvent {}

impl MinecartTransferEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_minecart_transfer<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MinecartTransferEvent) -> V + Send + Sync + 'static,
    ) {
        register::<minecart_transfer, V>(handler);
    }
}

hook!(
    minecart_pickup,
    MinecartPickupEventData => MinecartPickupEvent,
    |raw: &MinecartPickupEventData| MinecartPickupEvent {
        entity: raw.entity,
        item_entity: raw.item_entity,
        item: raw.item,
        world: raw.world,
    }
);

impl Cancellable for MinecartPickupEvent {}

impl MinecartPickupEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_minecart_pickup<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &MinecartPickupEvent) -> V + Send + Sync + 'static,
    ) {
        register::<minecart_pickup, V>(handler);
    }
}

hook!(
    block_dispense,
    BlockDispenseEventData => BlockDispenseEvent,
    |raw: &BlockDispenseEventData| BlockDispenseEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        item: std::cell::Cell::new(raw.item),
        dropper: raw.dropper,
        world: raw.world,
    },
    |safe: BlockDispenseEvent, raw: &mut BlockDispenseEventData| {
        raw.item = safe.item.get();
    }
);

impl Cancellable for BlockDispenseEvent {}

impl BlockDispenseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_dispense<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockDispenseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_dispense, V>(handler);
    }
}

hook!(
    item_transfer,
    ItemTransferEventData => ItemTransferEvent,
    |raw: &ItemTransferEventData| ItemTransferEvent {
        from_x: raw.from_x,
        from_y: raw.from_y,
        from_z: raw.from_z,
        to_x: raw.to_x,
        to_y: raw.to_y,
        to_z: raw.to_z,
        item: raw.item,
        world: raw.world,
    }
);

impl Cancellable for ItemTransferEvent {}

impl ItemTransferEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_transfer<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemTransferEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_transfer, V>(handler);
    }
}

hook!(
    hopper_pickup,
    HopperPickupEventData => HopperPickupEvent,
    |raw: &HopperPickupEventData| HopperPickupEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        entity: raw.entity,
        item: raw.item,
        world: raw.world,
    }
);

impl Cancellable for HopperPickupEvent {}

impl HopperPickupEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_hopper_pickup<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &HopperPickupEvent) -> V + Send + Sync + 'static,
    ) {
        register::<hopper_pickup, V>(handler);
    }
}

hook!(
    tripwire,
    TripwireEventData => TripwireEvent,
    |raw: &TripwireEventData| TripwireEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        powered: raw.powered,
        player: ffi_string(raw.player),
        world: raw.world,
    }
);

impl Cancellable for TripwireEvent {}

impl TripwireEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_tripwire<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &TripwireEvent) -> V + Send + Sync + 'static,
    ) {
        register::<tripwire, V>(handler);
    }
}

hook!(
    detector_rail,
    DetectorRailEventData => DetectorRailEvent,
    |raw: &DetectorRailEventData| DetectorRailEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        entity: raw.entity,
        active: raw.active,
        world: raw.world,
    }
);

impl Cancellable for DetectorRailEvent {}

impl DetectorRailEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_detector_rail<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DetectorRailEvent) -> V + Send + Sync + 'static,
    ) {
        register::<detector_rail, V>(handler);
    }
}

hook!(
    sculk_vibration,
    SculkVibrationEventData => SculkVibrationEvent,
    |raw: &SculkVibrationEventData| SculkVibrationEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        source_x: raw.source_x,
        source_y: raw.source_y,
        source_z: raw.source_z,
        frequency: std::cell::Cell::new(raw.frequency),
        power: std::cell::Cell::new(raw.power),
        world: raw.world,
    },
    |safe: SculkVibrationEvent, raw: &mut SculkVibrationEventData| {
        raw.frequency = safe.frequency.get();
        raw.power = safe.power.get();
    }
);

impl Cancellable for SculkVibrationEvent {}

impl SculkVibrationEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_sculk_vibration<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &SculkVibrationEvent) -> V + Send + Sync + 'static,
    ) {
        register::<sculk_vibration, V>(handler);
    }
}

hook!(
    crafter_craft,
    CrafterCraftEventData => CrafterCraftEvent,
    |raw: &CrafterCraftEventData| CrafterCraftEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        recipe: raw.recipe,
        item: std::cell::Cell::new(raw.item),
        count: std::cell::Cell::new(raw.count),
        world: raw.world,
    },
    |safe: CrafterCraftEvent, raw: &mut CrafterCraftEventData| {
        raw.item = safe.item.get();
        raw.count = safe.count.get();
    }
);

impl Cancellable for CrafterCraftEvent {}

impl CrafterCraftEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_crafter_craft<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &CrafterCraftEvent) -> V + Send + Sync + 'static,
    ) {
        register::<crafter_craft, V>(handler);
    }
}

hook!(
    dispenser_equip,
    DispenserEquipEventData => DispenserEquipEvent,
    |raw: &DispenserEquipEventData| DispenserEquipEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        player: ffi_string(raw.player),
        item: raw.item,
        slot: raw.slot,
        world: raw.world,
    }
);

impl Cancellable for DispenserEquipEvent {}

impl DispenserEquipEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_dispenser_equip<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DispenserEquipEvent) -> V + Send + Sync + 'static,
    ) {
        register::<dispenser_equip, V>(handler);
    }
}

hook!(
    pressure_plate,
    PressurePlateEventData => PressurePlateEvent,
    |raw: &PressurePlateEventData| PressurePlateEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        signal: std::cell::Cell::new(raw.signal),
        player: ffi_string(raw.player),
        world: raw.world,
    },
    |safe: PressurePlateEvent, raw: &mut PressurePlateEventData| {
        raw.signal = safe.signal.get();
    }
);

impl Cancellable for PressurePlateEvent {}

impl PressurePlateEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_pressure_plate<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PressurePlateEvent) -> V + Send + Sync + 'static,
    ) {
        register::<pressure_plate, V>(handler);
    }
}
