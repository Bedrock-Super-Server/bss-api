use crate::event::block::{
    BeaconEffectEvent, BlockFallEvent, BlockFormEvent, BlockGrowEvent, BlockIgniteEvent,
    BlockLandEvent, BlockUseEvent, BrewEvent, FuelBurnEvent, ItemCookEvent, JukeboxChangeEvent,
    LecternChangeEvent, LecternPageEvent, LiquidFlowEvent, NotePlayEvent, SculkShriekEvent,
    SignChangeEvent, TntPrimeEvent, TreeGrowEvent,
};
use crate::ffi::behaviour::{
    BeaconEffectEventData, BlockFallEventData, BlockFormEventData, BlockGrowEventData,
    BlockIgniteEventData, BlockLandEventData, BlockUseEventData, BrewEventData, FuelBurnEventData,
    ItemCookEventData, JukeboxChangeEventData, LecternChangeEventData, LecternPageEventData,
    LiquidFlowEventData, NotePlayEventData, SculkShriekEventData, SignChangeEventData,
    TntPrimeEventData, TreeGrowEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

thread_local! {
    static SIGN_CHANGE_TEXT: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

hook!(
    sign_change,
    SignChangeEventData => SignChangeEvent,
    |raw: &SignChangeEventData| SignChangeEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        front: raw.front,
        text: std::cell::RefCell::new(ffi_string(raw.text)),
    },
    |safe: SignChangeEvent, raw: &mut SignChangeEventData| {
        raw.text = crate::registry::hand_back(&SIGN_CHANGE_TEXT, safe.text.into_inner());
    }
);

impl Cancellable for SignChangeEvent {}

impl Server {
    /**
    Calls `handler` when a player finishes editing the text of a sign.

    Returning `true` leaves the sign unchanged.

    See [`SignChangeEvent`].
    */
    pub fn on_sign_change(
        &self,
        handler: impl Fn(&Server, &SignChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<sign_change>(handler);
    }
}

hook!(
    block_use,
    BlockUseEventData => BlockUseEvent,
    |raw: &BlockUseEventData| BlockUseEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        item: raw.item,
        action: ffi_string(raw.action),
    }
);

impl Cancellable for BlockUseEvent {}

impl Server {
    /**
    Calls `handler` when a player uses a block: opens a door or a container, presses a button, flips a lever, rings a bell...

    Returning `true` cancels the use: the block does not react.

    See [`BlockUseEvent`].
    */
    pub fn on_block_use(
        &self,
        handler: impl Fn(&Server, &BlockUseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_use>(handler);
    }
}

hook!(
    block_grow,
    BlockGrowEventData => BlockGrowEvent,
    |raw: &BlockGrowEventData| BlockGrowEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        new_block: std::cell::Cell::new(raw.new_block),
        bone_meal: raw.bone_meal,
        world: raw.world,
    },
    |safe: BlockGrowEvent, raw: &mut BlockGrowEventData| {
        raw.new_block = safe.new_block.get();
    }
);

impl Cancellable for BlockGrowEvent {}

impl Server {
    /**
    Calls `handler` when a plant grows a stage: crops, stems, sweet berries, cocoa, cactus, sugar cane...

    Returning `true` stops this growth step.

    See [`BlockGrowEvent`].
    */
    pub fn on_block_grow(
        &self,
        handler: impl Fn(&Server, &BlockGrowEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_grow>(handler);
    }
}

hook!(
    tree_grow,
    TreeGrowEventData => TreeGrowEvent,
    |raw: &TreeGrowEventData| TreeGrowEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        sapling: raw.sapling,
        world: raw.world,
    }
);

impl Cancellable for TreeGrowEvent {}

impl Server {
    /**
    Calls `handler` when a sapling or a fungus grows into a tree.

    Returning `true` keeps the sapling as it is.

    See [`TreeGrowEvent`].
    */
    pub fn on_tree_grow(
        &self,
        handler: impl Fn(&Server, &TreeGrowEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<tree_grow>(handler);
    }
}

hook!(
    block_fall,
    BlockFallEventData => BlockFallEvent,
    |raw: &BlockFallEventData| BlockFallEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        world: raw.world,
    }
);

impl Cancellable for BlockFallEvent {}

impl Server {
    /**
    Calls `handler` when a block affected by gravity, such as sand or an anvil, starts to fall.

    Returning `true` keeps the block in place.

    See [`BlockFallEvent`].
    */
    pub fn on_block_fall(
        &self,
        handler: impl Fn(&Server, &BlockFallEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_fall>(handler);
    }
}

hook!(
    block_land,
    BlockLandEventData => BlockLandEvent,
    |raw: &BlockLandEventData| BlockLandEvent {
        entity: raw.entity,
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: std::cell::Cell::new(raw.block),
        world: raw.world,
    },
    |safe: BlockLandEvent, raw: &mut BlockLandEventData| {
        raw.block = safe.block.get();
    }
);

impl Cancellable for BlockLandEvent {}

impl Server {
    /**
    Calls `handler` when a falling block lands and is about to become a block again.

    Returning `true` drops the block as an item instead of placing it.

    See [`BlockLandEvent`].
    */
    pub fn on_block_land(
        &self,
        handler: impl Fn(&Server, &BlockLandEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_land>(handler);
    }
}

hook!(
    block_form,
    BlockFormEventData => BlockFormEvent,
    |raw: &BlockFormEventData| BlockFormEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        new_block: std::cell::Cell::new(raw.new_block),
        world: raw.world,
    },
    |safe: BlockFormEvent, raw: &mut BlockFormEventData| {
        raw.new_block = safe.new_block.get();
    }
);

impl Cancellable for BlockFormEvent {}

impl Server {
    /**
    Calls `handler` when a block forms in the world on its own: lava and water make obsidian or cobblestone, snow piles up, ice freezes...

    Returning `true` prevents the block from forming.

    See [`BlockFormEvent`].
    */
    pub fn on_block_form(
        &self,
        handler: impl Fn(&Server, &BlockFormEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_form>(handler);
    }
}

hook!(
    block_ignite,
    BlockIgniteEventData => BlockIgniteEvent,
    |raw: &BlockIgniteEventData| BlockIgniteEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        block: raw.block,
        cause: ffi_string(raw.cause),
    }
);

impl Cancellable for BlockIgniteEvent {}

impl Server {
    /**
    Calls `handler` when a block is set on fire, or a block that can be lit is lit: fire, a campfire, a candle, TNT...

    Returning `true` prevents the ignition.

    See [`BlockIgniteEvent`].
    */
    pub fn on_block_ignite(
        &self,
        handler: impl Fn(&Server, &BlockIgniteEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<block_ignite>(handler);
    }
}

hook!(
    tnt_prime,
    TntPrimeEventData => TntPrimeEvent,
    |raw: &TntPrimeEventData| TntPrimeEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        fuse: std::cell::Cell::new(raw.fuse),
        cause: ffi_string(raw.cause),
        owner: raw.owner,
        world: raw.world,
    },
    |safe: TntPrimeEvent, raw: &mut TntPrimeEventData| {
        raw.fuse = safe.fuse.get();
    }
);

impl Cancellable for TntPrimeEvent {}

impl Server {
    /**
    Calls `handler` when tNT is primed and starts its fuse.

    Returning `true` keeps the TNT block unprimed.

    See [`TntPrimeEvent`].
    */
    pub fn on_tnt_prime(
        &self,
        handler: impl Fn(&Server, &TntPrimeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<tnt_prime>(handler);
    }
}

hook!(
    note_play,
    NotePlayEventData => NotePlayEvent,
    |raw: &NotePlayEventData| NotePlayEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        instrument: std::cell::Cell::new(raw.instrument),
        pitch: std::cell::Cell::new(raw.pitch),
    },
    |safe: NotePlayEvent, raw: &mut NotePlayEventData| {
        raw.instrument = safe.instrument.get();
        raw.pitch = safe.pitch.get();
    }
);

impl Cancellable for NotePlayEvent {}

impl Server {
    /**
    Calls `handler` when a note block plays a note, because a player hit it or it received a redstone signal.

    Returning `true` keeps the note block silent.

    See [`NotePlayEvent`].
    */
    pub fn on_note_play(
        &self,
        handler: impl Fn(&Server, &NotePlayEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<note_play>(handler);
    }
}

hook!(
    jukebox_change,
    JukeboxChangeEventData => JukeboxChangeEvent,
    |raw: &JukeboxChangeEventData| JukeboxChangeEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        disc: raw.disc,
        inserted: raw.inserted,
    }
);

impl Cancellable for JukeboxChangeEvent {}

impl Server {
    /**
    Calls `handler` when a player inserts a music disc into a jukebox or takes it out.

    Returning `true` leaves the jukebox as it was.

    See [`JukeboxChangeEvent`].
    */
    pub fn on_jukebox_change(
        &self,
        handler: impl Fn(&Server, &JukeboxChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<jukebox_change>(handler);
    }
}

hook!(
    lectern_change,
    LecternChangeEventData => LecternChangeEvent,
    |raw: &LecternChangeEventData| LecternChangeEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        book: raw.book,
        placed: raw.placed,
    }
);

impl Cancellable for LecternChangeEvent {}

impl Server {
    /**
    Calls `handler` when a player places a book on a lectern or takes it.

    Returning `true` leaves the lectern as it was.

    See [`LecternChangeEvent`].
    */
    pub fn on_lectern_change(
        &self,
        handler: impl Fn(&Server, &LecternChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<lectern_change>(handler);
    }
}

hook!(
    lectern_page,
    LecternPageEventData => LecternPageEvent,
    |raw: &LecternPageEventData| LecternPageEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        page: std::cell::Cell::new(raw.page),
    },
    |safe: LecternPageEvent, raw: &mut LecternPageEventData| {
        raw.page = safe.page.get();
    }
);

impl Cancellable for LecternPageEvent {}

impl Server {
    /**
    Calls `handler` when a player turns the page of a book on a lectern.

    Returning `true` keeps the current page.

    See [`LecternPageEvent`].
    */
    pub fn on_lectern_page(
        &self,
        handler: impl Fn(&Server, &LecternPageEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<lectern_page>(handler);
    }
}

hook!(
    item_cook,
    ItemCookEventData => ItemCookEvent,
    |raw: &ItemCookEventData| ItemCookEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        station: ffi_string(raw.station),
        input: raw.input,
        output: std::cell::Cell::new(raw.output),
        world: raw.world,
    },
    |safe: ItemCookEvent, raw: &mut ItemCookEventData| {
        raw.output = safe.output.get();
    }
);

impl Cancellable for ItemCookEvent {}

impl Server {
    /**
    Calls `handler` when a furnace, smoker, blast furnace or campfire finishes cooking an item.

    Returning `true` cancels the result: the input stays in place.

    See [`ItemCookEvent`].
    */
    pub fn on_item_cook(
        &self,
        handler: impl Fn(&Server, &ItemCookEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_cook>(handler);
    }
}

hook!(
    fuel_burn,
    FuelBurnEventData => FuelBurnEvent,
    |raw: &FuelBurnEventData| FuelBurnEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        station: ffi_string(raw.station),
        fuel: raw.fuel,
        burn_ticks: std::cell::Cell::new(raw.burn_ticks),
        world: raw.world,
    },
    |safe: FuelBurnEvent, raw: &mut FuelBurnEventData| {
        raw.burn_ticks = safe.burn_ticks.get();
    }
);

impl Cancellable for FuelBurnEvent {}

impl Server {
    /**
    Calls `handler` when a furnace, smoker or blast furnace starts burning a fuel item.

    Returning `true` keeps the fuel unburnt.

    See [`FuelBurnEvent`].
    */
    pub fn on_fuel_burn(
        &self,
        handler: impl Fn(&Server, &FuelBurnEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<fuel_burn>(handler);
    }
}

hook!(
    brew,
    BrewEventData => BrewEvent,
    |raw: &BrewEventData| BrewEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        ingredient: raw.ingredient,
        world: raw.world,
    }
);

impl Cancellable for BrewEvent {}

impl Server {
    /**
    Calls `handler` when a brewing stand finishes brewing.

    Returning `true` cancels the brew.

    See [`BrewEvent`].
    */
    pub fn on_brew(&self, handler: impl Fn(&Server, &BrewEvent) -> bool + Send + Sync + 'static) {
        register::<brew>(handler);
    }
}

hook!(
    beacon_effect,
    BeaconEffectEventData => BeaconEffectEvent,
    |raw: &BeaconEffectEventData| BeaconEffectEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        effect: std::cell::Cell::new(raw.effect),
        amplifier: std::cell::Cell::new(raw.amplifier),
        ticks: std::cell::Cell::new(raw.ticks),
    },
    |safe: BeaconEffectEvent, raw: &mut BeaconEffectEventData| {
        raw.effect = safe.effect.get();
        raw.amplifier = safe.amplifier.get();
        raw.ticks = safe.ticks.get();
    }
);

impl Cancellable for BeaconEffectEvent {}

impl Server {
    /**
    Calls `handler` when a beacon is about to give its effect to a player in range.

    Returning `true` withholds the effect this time.

    See [`BeaconEffectEvent`].
    */
    pub fn on_beacon_effect(
        &self,
        handler: impl Fn(&Server, &BeaconEffectEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<beacon_effect>(handler);
    }
}

hook!(
    liquid_flow,
    LiquidFlowEventData => LiquidFlowEvent,
    |raw: &LiquidFlowEventData| LiquidFlowEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        liquid: raw.liquid,
        world: raw.world,
    }
);

impl Cancellable for LiquidFlowEvent {}

impl Server {
    /**
    Calls `handler` when water or lava is about to flow into a block.

    Returning `true` stops the liquid from flowing into this block.

    See [`LiquidFlowEvent`].
    */
    pub fn on_liquid_flow(
        &self,
        handler: impl Fn(&Server, &LiquidFlowEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<liquid_flow>(handler);
    }
}

hook!(
    sculk_shriek,
    SculkShriekEventData => SculkShriekEvent,
    |raw: &SculkShriekEventData| SculkShriekEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        player: ffi_string(raw.player),
        warning: std::cell::Cell::new(raw.warning),
        world: raw.world,
    },
    |safe: SculkShriekEvent, raw: &mut SculkShriekEventData| {
        raw.warning = safe.warning.get();
    }
);

impl Cancellable for SculkShriekEvent {}

impl Server {
    /**
    Calls `handler` when a sculk shrieker shrieks because a player stepped near it or made a vibration it heard.

    Returning `true` keeps the shrieker silent.

    See [`SculkShriekEvent`].
    */
    pub fn on_sculk_shriek(
        &self,
        handler: impl Fn(&Server, &SculkShriekEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<sculk_shriek>(handler);
    }
}
