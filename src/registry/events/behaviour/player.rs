use crate::event::player::{
    BookEditEvent, CrossbowLoadEvent, DduiCloseEvent, DduiResponseEvent, ExperienceChangeEvent,
    ExperiencePickupEvent, FishEvent, FoodChangeEvent, HornBlowEvent, ItemBreakEvent,
    ItemDamageEvent, ItemMendEvent, OffhandChangeEvent, PlayerAfkEvent, PlayerAttackEvent,
    PlayerBedEnterEvent, PlayerBedLeaveEvent, PlayerChunkEnterEvent, PlayerCombustEvent,
    PlayerCrawlEvent, PlayerDamageEvent, PlayerEffectAddEvent, PlayerEffectRemoveEvent,
    PlayerFallEvent, PlayerFlyToggleEvent, PlayerHealEvent, PlayerJumpEvent, PlayerKickEvent,
    PlayerMountEvent, PlayerSwingMissEvent, PlayerViewDistanceChangeEvent, PlayerWarpEvent,
    PlayerWorldChangeEvent, ResourcePackStatusEvent, ShieldBlockEvent, TotemUseEvent,
    TridentRiptideEvent, VehicleMoveEvent,
};
use crate::ffi::behaviour::{
    BookEditEventData, CrossbowLoadEventData, DduiCloseEventData, DduiResponseEventData,
    ExperienceChangeEventData, ExperiencePickupEventData, FishEventData, FoodChangeEventData,
    HornBlowEventData, ItemBreakEventData, ItemDamageEventData, ItemMendEventData,
    OffhandChangeEventData, PlayerAfkEventData, PlayerAttackEventData, PlayerBedEnterEventData,
    PlayerBedLeaveEventData, PlayerChunkEnterEventData, PlayerCombustEventData,
    PlayerCrawlEventData, PlayerDamageEventData, PlayerEffectAddEventData,
    PlayerEffectRemoveEventData, PlayerFallEventData, PlayerFlyToggleEventData,
    PlayerHealEventData, PlayerJumpEventData, PlayerKickEventData, PlayerMountEventData,
    PlayerSwingMissEventData, PlayerViewDistanceChangeEventData, PlayerWarpEventData,
    PlayerWorldChangeEventData, ResourcePackStatusEventData, ShieldBlockEventData,
    TotemUseEventData, TridentRiptideEventData, VehicleMoveEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    player_damage,
    PlayerDamageEventData => PlayerDamageEvent,
    |raw: &PlayerDamageEventData| PlayerDamageEvent {
        player: ffi_string(raw.player),
        source: ffi_string(raw.source),
        damage: std::cell::Cell::new(raw.damage),
        attacker: ffi_string(raw.attacker),
    },
    |safe: PlayerDamageEvent, raw: &mut PlayerDamageEventData| {
        raw.damage = safe.damage.get();
    }
);

impl Cancellable for PlayerDamageEvent {}

impl PlayerDamageEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_damage<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerDamageEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_damage, V>(handler);
    }
}

hook!(
    player_heal,
    PlayerHealEventData => PlayerHealEvent,
    |raw: &PlayerHealEventData| PlayerHealEvent {
        player: ffi_string(raw.player),
        amount: std::cell::Cell::new(raw.amount),
        reason: ffi_string(raw.reason),
    },
    |safe: PlayerHealEvent, raw: &mut PlayerHealEventData| {
        raw.amount = safe.amount.get();
    }
);

impl Cancellable for PlayerHealEvent {}

impl PlayerHealEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_heal<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerHealEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_heal, V>(handler);
    }
}

hook!(
    player_combust,
    PlayerCombustEventData => PlayerCombustEvent,
    |raw: &PlayerCombustEventData| PlayerCombustEvent {
        player: ffi_string(raw.player),
        ticks: std::cell::Cell::new(raw.ticks),
    },
    |safe: PlayerCombustEvent, raw: &mut PlayerCombustEventData| {
        raw.ticks = safe.ticks.get();
    }
);

impl Cancellable for PlayerCombustEvent {}

impl PlayerCombustEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_combust<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerCombustEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_combust, V>(handler);
    }
}

hook!(
    food_change,
    FoodChangeEventData => FoodChangeEvent,
    |raw: &FoodChangeEventData| FoodChangeEvent {
        player: ffi_string(raw.player),
        old_food: raw.old_food,
        food: std::cell::Cell::new(raw.food),
        saturation: std::cell::Cell::new(raw.saturation),
    },
    |safe: FoodChangeEvent, raw: &mut FoodChangeEventData| {
        raw.food = safe.food.get();
        raw.saturation = safe.saturation.get();
    }
);

impl Cancellable for FoodChangeEvent {}

impl FoodChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_food_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &FoodChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<food_change, V>(handler);
    }
}

hook!(
    experience_change,
    ExperienceChangeEventData => ExperienceChangeEvent,
    |raw: &ExperienceChangeEventData| ExperienceChangeEvent {
        player: ffi_string(raw.player),
        level: raw.level,
        total: raw.total,
        amount: std::cell::Cell::new(raw.amount),
    },
    |safe: ExperienceChangeEvent, raw: &mut ExperienceChangeEventData| {
        raw.amount = safe.amount.get();
    }
);

impl Cancellable for ExperienceChangeEvent {}

impl ExperienceChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_experience_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ExperienceChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<experience_change, V>(handler);
    }
}

hook!(
    experience_pickup,
    ExperiencePickupEventData => ExperiencePickupEvent,
    |raw: &ExperiencePickupEventData| ExperiencePickupEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        amount: std::cell::Cell::new(raw.amount),
    },
    |safe: ExperiencePickupEvent, raw: &mut ExperiencePickupEventData| {
        raw.amount = safe.amount.get();
    }
);

impl Cancellable for ExperiencePickupEvent {}

impl ExperiencePickupEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_experience_pickup<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ExperiencePickupEvent) -> V + Send + Sync + 'static,
    ) {
        register::<experience_pickup, V>(handler);
    }
}

hook!(
    item_mend,
    ItemMendEventData => ItemMendEvent,
    |raw: &ItemMendEventData| ItemMendEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        repair: std::cell::Cell::new(raw.repair),
    },
    |safe: ItemMendEvent, raw: &mut ItemMendEventData| {
        raw.repair = safe.repair.get();
    }
);

impl Cancellable for ItemMendEvent {}

impl ItemMendEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_mend<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemMendEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_mend, V>(handler);
    }
}

hook!(
    item_damage,
    ItemDamageEventData => ItemDamageEvent,
    |raw: &ItemDamageEventData| ItemDamageEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        slot: raw.slot,
        damage: std::cell::Cell::new(raw.damage),
    },
    |safe: ItemDamageEvent, raw: &mut ItemDamageEventData| {
        raw.damage = safe.damage.get();
    }
);

impl Cancellable for ItemDamageEvent {}

impl ItemDamageEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_damage<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemDamageEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_damage, V>(handler);
    }
}

hook!(
    item_break,
    ItemBreakEventData => ItemBreakEvent,
    |raw: &ItemBreakEventData| ItemBreakEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        slot: raw.slot,
    }
);

impl Cancellable for ItemBreakEvent {}

impl ItemBreakEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_item_break<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ItemBreakEvent) -> V + Send + Sync + 'static,
    ) {
        register::<item_break, V>(handler);
    }
}

hook!(
    shield_block,
    ShieldBlockEventData => ShieldBlockEvent,
    |raw: &ShieldBlockEventData| ShieldBlockEvent {
        player: ffi_string(raw.player),
        damage: raw.damage,
        attacker: ffi_string(raw.attacker),
    }
);

impl Cancellable for ShieldBlockEvent {}

impl ShieldBlockEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_shield_block<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ShieldBlockEvent) -> V + Send + Sync + 'static,
    ) {
        register::<shield_block, V>(handler);
    }
}

hook!(
    totem_use,
    TotemUseEventData => TotemUseEvent,
    |raw: &TotemUseEventData| TotemUseEvent {
        player: ffi_string(raw.player),
    }
);

impl Cancellable for TotemUseEvent {}

impl TotemUseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_totem_use<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &TotemUseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<totem_use, V>(handler);
    }
}

hook!(
    player_warp,
    PlayerWarpEventData => PlayerWarpEvent,
    |raw: &PlayerWarpEventData| PlayerWarpEvent {
        player: ffi_string(raw.player),
        cause: ffi_string(raw.cause),
        x: std::cell::Cell::new(raw.x),
        y: std::cell::Cell::new(raw.y),
        z: std::cell::Cell::new(raw.z),
    },
    |safe: PlayerWarpEvent, raw: &mut PlayerWarpEventData| {
        raw.x = safe.x.get();
        raw.y = safe.y.get();
        raw.z = safe.z.get();
    }
);

impl Cancellable for PlayerWarpEvent {}

impl PlayerWarpEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_warp<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerWarpEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_warp, V>(handler);
    }
}

hook!(
    player_mount,
    PlayerMountEventData => PlayerMountEvent,
    |raw: &PlayerMountEventData| PlayerMountEvent {
        player: ffi_string(raw.player),
        vehicle: raw.vehicle,
        mounted: raw.mounted,
    }
);

impl Cancellable for PlayerMountEvent {}

impl PlayerMountEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_mount<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerMountEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_mount, V>(handler);
    }
}

hook!(
    vehicle_move,
    VehicleMoveEventData => VehicleMoveEvent,
    |raw: &VehicleMoveEventData| VehicleMoveEvent {
        player: ffi_string(raw.player),
        vehicle: raw.vehicle,
        x: raw.x,
        y: raw.y,
        z: raw.z,
    }
);

impl Cancellable for VehicleMoveEvent {}

impl VehicleMoveEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_vehicle_move<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &VehicleMoveEvent) -> V + Send + Sync + 'static,
    ) {
        register::<vehicle_move, V>(handler);
    }
}

hook!(
    crossbow_load,
    CrossbowLoadEventData => CrossbowLoadEvent,
    |raw: &CrossbowLoadEventData| CrossbowLoadEvent {
        player: ffi_string(raw.player),
        ammo: raw.ammo,
    }
);

impl Cancellable for CrossbowLoadEvent {}

impl CrossbowLoadEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_crossbow_load<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &CrossbowLoadEvent) -> V + Send + Sync + 'static,
    ) {
        register::<crossbow_load, V>(handler);
    }
}

hook!(
    trident_riptide,
    TridentRiptideEventData => TridentRiptideEvent,
    |raw: &TridentRiptideEventData| TridentRiptideEvent {
        player: ffi_string(raw.player),
        strength: std::cell::Cell::new(raw.strength),
    },
    |safe: TridentRiptideEvent, raw: &mut TridentRiptideEventData| {
        raw.strength = safe.strength.get();
    }
);

impl Cancellable for TridentRiptideEvent {}

impl TridentRiptideEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_trident_riptide<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &TridentRiptideEvent) -> V + Send + Sync + 'static,
    ) {
        register::<trident_riptide, V>(handler);
    }
}

hook!(
    fish,
    FishEventData => FishEvent,
    |raw: &FishEventData| FishEvent {
        player: ffi_string(raw.player),
        action: ffi_string(raw.action),
        item: std::cell::Cell::new(raw.item),
        experience: std::cell::Cell::new(raw.experience),
        hooked: raw.hooked,
    },
    |safe: FishEvent, raw: &mut FishEventData| {
        raw.item = safe.item.get();
        raw.experience = safe.experience.get();
    }
);

impl Cancellable for FishEvent {}

impl FishEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_fish<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &FishEvent) -> V + Send + Sync + 'static,
    ) {
        register::<fish, V>(handler);
    }
}

hook!(
    horn_blow,
    HornBlowEventData => HornBlowEvent,
    |raw: &HornBlowEventData| HornBlowEvent {
        player: ffi_string(raw.player),
        variant: std::cell::Cell::new(raw.variant),
    },
    |safe: HornBlowEvent, raw: &mut HornBlowEventData| {
        raw.variant = safe.variant.get();
    }
);

impl Cancellable for HornBlowEvent {}

impl HornBlowEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_horn_blow<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &HornBlowEvent) -> V + Send + Sync + 'static,
    ) {
        register::<horn_blow, V>(handler);
    }
}

hook!(
    book_edit,
    BookEditEventData => BookEditEvent,
    |raw: &BookEditEventData| BookEditEvent {
        player: ffi_string(raw.player),
        slot: raw.slot,
        action: ffi_string(raw.action),
        page: raw.page,
        text: ffi_string(raw.text),
    }
);

impl Cancellable for BookEditEvent {}

impl BookEditEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_book_edit<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &BookEditEvent) -> V + Send + Sync + 'static,
    ) {
        register::<book_edit, V>(handler);
    }
}

hook!(
    offhand_change,
    OffhandChangeEventData => OffhandChangeEvent,
    |raw: &OffhandChangeEventData| OffhandChangeEvent {
        player: ffi_string(raw.player),
        item: raw.item,
        count: raw.count,
    }
);

impl Cancellable for OffhandChangeEvent {}

impl OffhandChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_offhand_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &OffhandChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<offhand_change, V>(handler);
    }
}

hook!(
    player_bed_enter,
    PlayerBedEnterEventData => PlayerBedEnterEvent,
    |raw: &PlayerBedEnterEventData| PlayerBedEnterEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        world: raw.world,
    }
);

impl Cancellable for PlayerBedEnterEvent {}

impl PlayerBedEnterEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_bed_enter<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerBedEnterEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_bed_enter, V>(handler);
    }
}

hook!(
    player_bed_leave,
    PlayerBedLeaveEventData => PlayerBedLeaveEvent,
    |raw: &PlayerBedLeaveEventData| PlayerBedLeaveEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        cause: ffi_string(raw.cause),
        world: raw.world,
    }
);

impl Cancellable for PlayerBedLeaveEvent {}

impl PlayerBedLeaveEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_bed_leave<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerBedLeaveEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_bed_leave, V>(handler);
    }
}

hook!(
    player_view_distance_change,
    PlayerViewDistanceChangeEventData => PlayerViewDistanceChangeEvent,
    |raw: &PlayerViewDistanceChangeEventData| PlayerViewDistanceChangeEvent {
        player: ffi_string(raw.player),
        requested: raw.requested,
        old_radius: raw.old_radius,
        radius: std::cell::Cell::new(raw.radius),
    },
    |safe: PlayerViewDistanceChangeEvent, raw: &mut PlayerViewDistanceChangeEventData| {
        raw.radius = safe.radius.get();
    }
);

impl Cancellable for PlayerViewDistanceChangeEvent {}

impl PlayerViewDistanceChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_view_distance_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerViewDistanceChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_view_distance_change, V>(handler);
    }
}

hook!(
    player_kick,
    PlayerKickEventData => PlayerKickEvent,
    |raw: &PlayerKickEventData| PlayerKickEvent {
        player: ffi_string(raw.player),
        reason: ffi_string(raw.reason),
    }
);

impl Cancellable for PlayerKickEvent {}

impl PlayerKickEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_kick<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerKickEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_kick, V>(handler);
    }
}

hook!(
    player_jump,
    PlayerJumpEventData => PlayerJumpEvent,
    |raw: &PlayerJumpEventData| PlayerJumpEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
    }
);

impl Cancellable for PlayerJumpEvent {}

impl PlayerJumpEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_jump<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerJumpEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_jump, V>(handler);
    }
}

hook!(
    player_swing_miss,
    PlayerSwingMissEventData => PlayerSwingMissEvent,
    |raw: &PlayerSwingMissEventData| PlayerSwingMissEvent {
        player: ffi_string(raw.player),
        x: raw.x,
        y: raw.y,
        z: raw.z,
        pitch: raw.pitch,
        yaw: raw.yaw,
    }
);

impl Cancellable for PlayerSwingMissEvent {}

impl PlayerSwingMissEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_swing_miss<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerSwingMissEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_swing_miss, V>(handler);
    }
}

hook!(
    player_fly_toggle,
    PlayerFlyToggleEventData => PlayerFlyToggleEvent,
    |raw: &PlayerFlyToggleEventData| PlayerFlyToggleEvent {
        player: ffi_string(raw.player),
        flying: raw.flying,
    }
);

impl Cancellable for PlayerFlyToggleEvent {}

impl PlayerFlyToggleEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_fly_toggle<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerFlyToggleEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_fly_toggle, V>(handler);
    }
}

hook!(
    player_crawl,
    PlayerCrawlEventData => PlayerCrawlEvent,
    |raw: &PlayerCrawlEventData| PlayerCrawlEvent {
        player: ffi_string(raw.player),
        crawling: raw.crawling,
    }
);

impl Cancellable for PlayerCrawlEvent {}

impl PlayerCrawlEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_crawl<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerCrawlEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_crawl, V>(handler);
    }
}

hook!(
    player_attack,
    PlayerAttackEventData => PlayerAttackEvent,
    |raw: &PlayerAttackEventData| PlayerAttackEvent {
        player: ffi_string(raw.player),
        entity: raw.entity,
        victim: ffi_string(raw.victim),
        item: raw.item,
    }
);

impl Cancellable for PlayerAttackEvent {}

impl PlayerAttackEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_attack<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerAttackEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_attack, V>(handler);
    }
}

hook!(
    player_fall,
    PlayerFallEventData => PlayerFallEvent,
    |raw: &PlayerFallEventData| PlayerFallEvent {
        player: ffi_string(raw.player),
        distance: raw.distance,
        damage: std::cell::Cell::new(raw.damage),
        block: ffi_string(raw.block),
    },
    |safe: PlayerFallEvent, raw: &mut PlayerFallEventData| {
        raw.damage = safe.damage.get();
    }
);

impl Cancellable for PlayerFallEvent {}

impl PlayerFallEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_fall<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerFallEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_fall, V>(handler);
    }
}

hook!(
    player_world_change,
    PlayerWorldChangeEventData => PlayerWorldChangeEvent,
    |raw: &PlayerWorldChangeEventData| PlayerWorldChangeEvent {
        player: ffi_string(raw.player),
        from_world: raw.from_world,
        to_world: raw.to_world,
        cause: ffi_string(raw.cause),
    }
);

impl Cancellable for PlayerWorldChangeEvent {}

impl PlayerWorldChangeEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_world_change<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerWorldChangeEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_world_change, V>(handler);
    }
}

hook!(
    player_chunk_enter,
    PlayerChunkEnterEventData => PlayerChunkEnterEvent,
    |raw: &PlayerChunkEnterEventData| PlayerChunkEnterEvent {
        player: ffi_string(raw.player),
        chunk_x: raw.chunk_x,
        chunk_z: raw.chunk_z,
        from_x: raw.from_x,
        from_z: raw.from_z,
        world: raw.world,
    }
);

impl Cancellable for PlayerChunkEnterEvent {}

impl PlayerChunkEnterEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_chunk_enter<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerChunkEnterEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_chunk_enter, V>(handler);
    }
}

hook!(
    resource_pack_status,
    ResourcePackStatusEventData => ResourcePackStatusEvent,
    |raw: &ResourcePackStatusEventData| ResourcePackStatusEvent {
        player: ffi_string(raw.player),
        status: ffi_string(raw.status),
    }
);

impl Cancellable for ResourcePackStatusEvent {}

impl ResourcePackStatusEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_resource_pack_status<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ResourcePackStatusEvent) -> V + Send + Sync + 'static,
    ) {
        register::<resource_pack_status, V>(handler);
    }
}

hook!(
    player_afk,
    PlayerAfkEventData => PlayerAfkEvent,
    |raw: &PlayerAfkEventData| PlayerAfkEvent {
        player: ffi_string(raw.player),
        afk: raw.afk,
        idle: raw.idle,
    }
);

impl Cancellable for PlayerAfkEvent {}

impl PlayerAfkEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_afk<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerAfkEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_afk, V>(handler);
    }
}

hook!(
    player_effect_add,
    PlayerEffectAddEventData => PlayerEffectAddEvent,
    |raw: &PlayerEffectAddEventData| PlayerEffectAddEvent {
        player: ffi_string(raw.player),
        effect: ffi_string(raw.effect),
        amplifier: std::cell::Cell::new(raw.amplifier),
        duration: std::cell::Cell::new(raw.duration),
    },
    |safe: PlayerEffectAddEvent, raw: &mut PlayerEffectAddEventData| {
        raw.amplifier = safe.amplifier.get();
        raw.duration = safe.duration.get();
    }
);

impl Cancellable for PlayerEffectAddEvent {}

impl PlayerEffectAddEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_effect_add<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerEffectAddEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_effect_add, V>(handler);
    }
}

hook!(
    player_effect_remove,
    PlayerEffectRemoveEventData => PlayerEffectRemoveEvent,
    |raw: &PlayerEffectRemoveEventData| PlayerEffectRemoveEvent {
        player: ffi_string(raw.player),
        effect: ffi_string(raw.effect),
        amplifier: raw.amplifier,
    }
);

impl Cancellable for PlayerEffectRemoveEvent {}

impl PlayerEffectRemoveEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_effect_remove<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerEffectRemoveEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_effect_remove, V>(handler);
    }
}

hook!(
    ddui_response,
    DduiResponseEventData => DduiResponseEvent,
    |raw: &DduiResponseEventData| DduiResponseEvent {
        player: ffi_string(raw.player),
        form_id: raw.form_id,
        element: ffi_string(raw.element),
        kind: ffi_string(raw.kind),
        number: raw.number,
        flag: raw.flag,
        text: ffi_string(raw.text),
    }
);

impl Cancellable for DduiResponseEvent {}

impl DduiResponseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_ddui_response<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DduiResponseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<ddui_response, V>(handler);
    }
}

hook!(
    ddui_close,
    DduiCloseEventData => DduiCloseEvent,
    |raw: &DduiCloseEventData| DduiCloseEvent {
        player: ffi_string(raw.player),
        form_id: raw.form_id,
        reason: ffi_string(raw.reason),
    }
);

impl Cancellable for DduiCloseEvent {}

impl DduiCloseEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_ddui_close<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &DduiCloseEvent) -> V + Send + Sync + 'static,
    ) {
        register::<ddui_close, V>(handler);
    }
}
