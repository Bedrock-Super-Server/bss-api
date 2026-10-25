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
use crate::registry::hook::{Cancellable, hook, register};
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

impl Server {
    /**
    Calls `handler` when the redstone power of a block changes.

    Returning `true` keeps the previous power.

    See [`RedstonePowerEvent`].
    */
    pub fn on_redstone_power(
        &self,
        handler: impl Fn(&Server, &RedstonePowerEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<redstone_power>(handler);
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

impl Server {
    /**
    Calls `handler` when a piston extends or retracts.

    Returning `true` keeps the piston still.

    See [`PistonMoveEvent`].
    */
    pub fn on_piston_move(
        &self,
        handler: impl Fn(&Server, &PistonMoveEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<piston_move>(handler);
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

impl Server {
    /**
    Calls `handler` when a piston pushes an entity.

    Returning `true` leaves the entity where it is.

    See [`PistonPushEvent`].
    */
    pub fn on_piston_push(
        &self,
        handler: impl Fn(&Server, &PistonPushEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<piston_push>(handler);
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

impl Server {
    /**
    Calls `handler` when a minecart passes over a powered activator rail.

    Returning `true` ignores the rail.

    See [`MinecartActivateEvent`].
    */
    pub fn on_minecart_activate(
        &self,
        handler: impl Fn(&Server, &MinecartActivateEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<minecart_activate>(handler);
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

impl Server {
    /**
    Calls `handler` when a hopper moves an item into or out of a minecart.

    Returning `true` cancels the transfer.

    See [`MinecartTransferEvent`].
    */
    pub fn on_minecart_transfer(
        &self,
        handler: impl Fn(&Server, &MinecartTransferEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<minecart_transfer>(handler);
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

impl Server {
    /**
    Calls `handler` when a hopper minecart picks up an item entity.

    Returning `true` leaves the item where it is.

    See [`MinecartPickupEvent`].
    */
    pub fn on_minecart_pickup(
        &self,
        handler: impl Fn(&Server, &MinecartPickupEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<minecart_pickup>(handler);
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

impl Server {
    /**
    Calls `handler` when a dispenser or dropper fires an item.

    Returning `true` cancels the shot.

    See [`BlockDispenseEvent`].
    */
    pub fn on_block_dispense(
        &self,
        handler: impl Fn(&Server, &BlockDispenseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_dispense>(handler);
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

impl Server {
    /**
    Calls `handler` when a hopper moves an item from one container to another.

    Returning `true` cancels the transfer.

    See [`ItemTransferEvent`].
    */
    pub fn on_item_transfer(
        &self,
        handler: impl Fn(&Server, &ItemTransferEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_transfer>(handler);
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

impl Server {
    /**
    Calls `handler` when a hopper picks up an item entity lying above it.

    Returning `true` leaves the item where it is.

    See [`HopperPickupEvent`].
    */
    pub fn on_hopper_pickup(
        &self,
        handler: impl Fn(&Server, &HopperPickupEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<hopper_pickup>(handler);
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

impl Server {
    /**
    Calls `handler` when a tripwire is triggered or released.

    Returning `true` leaves the tripwire as it was.

    See [`TripwireEvent`].
    */
    pub fn on_tripwire(
        &self,
        handler: impl Fn(&Server, &TripwireEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<tripwire>(handler);
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

impl Server {
    /**
    Calls `handler` when a minecart turns a detector rail on or off.

    Returning `true` leaves the rail as it was.

    See [`DetectorRailEvent`].
    */
    pub fn on_detector_rail(
        &self,
        handler: impl Fn(&Server, &DetectorRailEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<detector_rail>(handler);
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

impl Server {
    /**
    Calls `handler` when a sculk sensor detects a vibration.

    Returning `true` keeps the sensor from reacting.

    See [`SculkVibrationEvent`].
    */
    pub fn on_sculk_vibration(
        &self,
        handler: impl Fn(&Server, &SculkVibrationEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<sculk_vibration>(handler);
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

impl Server {
    /**
    Calls `handler` when a crafter crafts an item.

    Returning `true` cancels the craft.

    See [`CrafterCraftEvent`].
    */
    pub fn on_crafter_craft(
        &self,
        handler: impl Fn(&Server, &CrafterCraftEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<crafter_craft>(handler);
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

impl Server {
    /**
    Calls `handler` when a dispenser puts armor on a player in front of it.

    Returning `true` keeps the armor in the dispenser.

    See [`DispenserEquipEvent`].
    */
    pub fn on_dispenser_equip(
        &self,
        handler: impl Fn(&Server, &DispenserEquipEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<dispenser_equip>(handler);
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

impl Server {
    /**
    Calls `handler` when something steps on or leaves a pressure plate.

    Returning `true` keeps the plate in its previous state.

    See [`PressurePlateEvent`].
    */
    pub fn on_pressure_plate(
        &self,
        handler: impl Fn(&Server, &PressurePlateEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<pressure_plate>(handler);
    }
}
