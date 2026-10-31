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
use crate::registry::hook::{Cancellable, Verdict, hook, register};
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

impl SignChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_sign_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &SignChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<sign_change, V>(handler);
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

impl BlockUseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_use<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockUseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_use, V>(handler);
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

impl BlockGrowEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_grow<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockGrowEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_grow, V>(handler);
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

impl TreeGrowEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_tree_grow<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &TreeGrowEvent) -> V + Send + Sync + 'static,
    ) {
        register::<tree_grow, V>(handler);
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

impl BlockFallEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_fall<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockFallEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_fall, V>(handler);
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

impl BlockLandEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_land<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockLandEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_land, V>(handler);
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

impl BlockFormEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_form<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockFormEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_form, V>(handler);
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

impl BlockIgniteEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_block_ignite<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BlockIgniteEvent) -> V + Send + Sync + 'static,
    ) {
        register::<block_ignite, V>(handler);
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

impl TntPrimeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_tnt_prime<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &TntPrimeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<tnt_prime, V>(handler);
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

impl NotePlayEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_note_play<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &NotePlayEvent) -> V + Send + Sync + 'static,
    ) {
        register::<note_play, V>(handler);
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

impl JukeboxChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_jukebox_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &JukeboxChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<jukebox_change, V>(handler);
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

impl LecternChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_lectern_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &LecternChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<lectern_change, V>(handler);
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

impl LecternPageEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_lectern_page<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &LecternPageEvent) -> V + Send + Sync + 'static,
    ) {
        register::<lectern_page, V>(handler);
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

impl ItemCookEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_cook<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemCookEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_cook, V>(handler);
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

impl FuelBurnEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_fuel_burn<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &FuelBurnEvent) -> V + Send + Sync + 'static,
    ) {
        register::<fuel_burn, V>(handler);
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

impl BrewEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_brew<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BrewEvent) -> V + Send + Sync + 'static,
    ) {
        register::<brew, V>(handler);
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

impl BeaconEffectEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_beacon_effect<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BeaconEffectEvent) -> V + Send + Sync + 'static,
    ) {
        register::<beacon_effect, V>(handler);
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

impl LiquidFlowEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_liquid_flow<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &LiquidFlowEvent) -> V + Send + Sync + 'static,
    ) {
        register::<liquid_flow, V>(handler);
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

impl SculkShriekEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_sculk_shriek<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &SculkShriekEvent) -> V + Send + Sync + 'static,
    ) {
        register::<sculk_shriek, V>(handler);
    }
}
