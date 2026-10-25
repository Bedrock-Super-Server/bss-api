use crate::event::player::{
    BookEditEvent, CrossbowLoadEvent, DduiCloseEvent, DduiResponseEvent, ExperienceChangeEvent,
    ExperiencePickupEvent, FishEvent, FoodChangeEvent, HornBlowEvent, ItemBreakEvent,
    ItemDamageEvent, ItemMendEvent, OffhandChangeEvent, PlayerBedEnterEvent, PlayerBedLeaveEvent,
    PlayerCombustEvent, PlayerDamageEvent, PlayerEffectAddEvent, PlayerEffectRemoveEvent,
    PlayerHealEvent, PlayerJumpEvent, PlayerKickEvent, PlayerMountEvent,
    PlayerViewDistanceChangeEvent, PlayerWarpEvent, ShieldBlockEvent, TotemUseEvent,
    TridentRiptideEvent, VehicleMoveEvent,
};
use crate::ffi::behaviour::{
    BookEditEventData, CrossbowLoadEventData, DduiCloseEventData, DduiResponseEventData,
    ExperienceChangeEventData, ExperiencePickupEventData, FishEventData, FoodChangeEventData,
    HornBlowEventData, ItemBreakEventData, ItemDamageEventData, ItemMendEventData,
    OffhandChangeEventData, PlayerBedEnterEventData, PlayerBedLeaveEventData,
    PlayerCombustEventData, PlayerDamageEventData, PlayerEffectAddEventData,
    PlayerEffectRemoveEventData, PlayerHealEventData, PlayerJumpEventData, PlayerKickEventData,
    PlayerMountEventData, PlayerViewDistanceChangeEventData, PlayerWarpEventData,
    ShieldBlockEventData, TotemUseEventData, TridentRiptideEventData, VehicleMoveEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
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

impl Server {
    /**
    Calls `handler` when a player takes damage.

    Returning `true` cancels the damage.

    See [`PlayerDamageEvent`].
    */
    pub fn on_player_damage(
        &self,
        handler: impl Fn(&Server, &PlayerDamageEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_damage>(handler);
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

impl Server {
    /**
    Calls `handler` when a player regains health.

    Returning `true` cancels the healing.

    See [`PlayerHealEvent`].
    */
    pub fn on_player_heal(
        &self,
        handler: impl Fn(&Server, &PlayerHealEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_heal>(handler);
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

impl Server {
    /**
    Calls `handler` when a player catches fire.

    Returning `true` keeps the player from burning.

    See [`PlayerCombustEvent`].
    */
    pub fn on_player_combust(
        &self,
        handler: impl Fn(&Server, &PlayerCombustEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_combust>(handler);
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

impl Server {
    /**
    Calls `handler` when a player's hunger bar changes.

    Returning `true` keeps the previous values.

    See [`FoodChangeEvent`].
    */
    pub fn on_food_change(
        &self,
        handler: impl Fn(&Server, &FoodChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<food_change>(handler);
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

impl Server {
    /**
    Calls `handler` when a player gains or loses experience.

    Returning `true` leaves the experience unchanged.

    See [`ExperienceChangeEvent`].
    */
    pub fn on_experience_change(
        &self,
        handler: impl Fn(&Server, &ExperienceChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<experience_change>(handler);
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

impl Server {
    /**
    Calls `handler` when a player picks up an experience orb.

    Returning `true` leaves the orb where it is.

    See [`ExperiencePickupEvent`].
    */
    pub fn on_experience_pickup(
        &self,
        handler: impl Fn(&Server, &ExperiencePickupEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<experience_pickup>(handler);
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

impl Server {
    /**
    Calls `handler` when mending repairs an item with experience the player picked up.

    Returning `true` cancels the repair.

    See [`ItemMendEvent`].
    */
    pub fn on_item_mend(
        &self,
        handler: impl Fn(&Server, &ItemMendEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_mend>(handler);
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

impl Server {
    /**
    Calls `handler` when a tool, weapon or piece of armor is about to lose durability.

    Returning `true` prevents the loss.

    See [`ItemDamageEvent`].
    */
    pub fn on_item_damage(
        &self,
        handler: impl Fn(&Server, &ItemDamageEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_damage>(handler);
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

impl Server {
    /**
    Calls `handler` when a tool, weapon or piece of armor breaks after losing its last durability point.

    The value returned is ignored: this event cannot be cancelled.

    See [`ItemBreakEvent`].
    */
    pub fn on_item_break(
        &self,
        handler: impl Fn(&Server, &ItemBreakEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<item_break>(handler);
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

impl Server {
    /**
    Calls `handler` when a player blocks an attack with a shield.

    Returning `true` cancels it.

    See [`ShieldBlockEvent`].
    */
    pub fn on_shield_block(
        &self,
        handler: impl Fn(&Server, &ShieldBlockEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<shield_block>(handler);
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

impl Server {
    /**
    Calls `handler` when a totem of undying saves a player from death.

    Returning `true` stops the totem from saving the player.

    See [`TotemUseEvent`].
    */
    pub fn on_totem_use(
        &self,
        handler: impl Fn(&Server, &TotemUseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<totem_use>(handler);
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

impl Server {
    /**
    Calls `handler` when a player teleports with an ender pearl or a chorus fruit.

    Returning `true` cancels the teleport.

    See [`PlayerWarpEvent`].
    */
    pub fn on_player_warp(
        &self,
        handler: impl Fn(&Server, &PlayerWarpEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_warp>(handler);
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

impl Server {
    /**
    Calls `handler` when a player gets on or off a vehicle or a mob.

    Returning `true` cancels it.

    See [`PlayerMountEvent`].
    */
    pub fn on_player_mount(
        &self,
        handler: impl Fn(&Server, &PlayerMountEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_mount>(handler);
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

impl Server {
    /**
    Calls `handler` when a player moves the vehicle they ride.

    Returning `true` keeps the vehicle where it was.

    See [`VehicleMoveEvent`].
    */
    pub fn on_vehicle_move(
        &self,
        handler: impl Fn(&Server, &VehicleMoveEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<vehicle_move>(handler);
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

impl Server {
    /**
    Calls `handler` when a player loads a crossbow.

    Returning `true` keeps the crossbow unloaded.

    See [`CrossbowLoadEvent`].
    */
    pub fn on_crossbow_load(
        &self,
        handler: impl Fn(&Server, &CrossbowLoadEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<crossbow_load>(handler);
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

impl Server {
    /**
    Calls `handler` when a player launches with a Riptide trident.

    Returning `true` cancels the launch.

    See [`TridentRiptideEvent`].
    */
    pub fn on_trident_riptide(
        &self,
        handler: impl Fn(&Server, &TridentRiptideEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<trident_riptide>(handler);
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

impl Server {
    /**
    Calls `handler` when a player casts a fishing rod, or reels it in.

    Returning `true` cancels the cast or the catch.

    See [`FishEvent`].
    */
    pub fn on_fish(&self, handler: impl Fn(&Server, &FishEvent) -> bool + Send + Sync + 'static) {
        register::<fish>(handler);
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

impl Server {
    /**
    Calls `handler` when a player blows a goat horn.

    Returning `true` keeps the horn silent.

    See [`HornBlowEvent`].
    */
    pub fn on_horn_blow(
        &self,
        handler: impl Fn(&Server, &HornBlowEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<horn_blow>(handler);
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

impl Server {
    /**
    Calls `handler` when a player edits or signs a book and quill.

    Returning `true` rejects the edit.

    See [`BookEditEvent`].
    */
    pub fn on_book_edit(
        &self,
        handler: impl Fn(&Server, &BookEditEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<book_edit>(handler);
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

impl Server {
    /**
    Calls `handler` when a player changes the item in their off hand.

    Returning `true` cancels it.

    See [`OffhandChangeEvent`].
    */
    pub fn on_offhand_change(
        &self,
        handler: impl Fn(&Server, &OffhandChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<offhand_change>(handler);
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

impl Server {
    /**
    Calls `handler` when a player lies down in a bed: it is night or a thunderstorm, no monster is near and nobody else lies there.

    Returning `true` from a handler keeps the player standing; the bed still becomes their respawn point.

    See [`PlayerBedEnterEvent`].
    */
    pub fn on_player_bed_enter(
        &self,
        handler: impl Fn(&Server, &PlayerBedEnterEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_bed_enter>(handler);
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

impl Server {
    /**
    Calls `handler` when a player gets out of bed. `cause` is `morning` when the world woke everyone up, `manual` when the player left by themselves, `damage`, `gamemode` or `travel` when something forced them out, and `bed_destroyed` when the bed is gone.

    Returning `true` changes nothing: the player leaves the bed anyway.

    See [`PlayerBedLeaveEvent`].
    */
    pub fn on_player_bed_leave(
        &self,
        handler: impl Fn(&Server, &PlayerBedLeaveEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_bed_leave>(handler);
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

impl Server {
    /**
    Calls `handler` when a player's client asks for another view distance, in chunks. `radius` is what the server grants, within its own `view_distance`; set another one to grant that instead.

    Returning `true` from a handler keeps the view distance the player has.

    See [`PlayerViewDistanceChangeEvent`].
    */
    pub fn on_player_view_distance_change(
        &self,
        handler: impl Fn(&Server, &PlayerViewDistanceChangeEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_view_distance_change>(handler);
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

impl Server {
    /**
    Calls `handler` when something asks to disconnect a player: a plugin, the `/kick` command or the server's own protections.

    Returning `true` from a handler keeps the player connected.

    See [`PlayerKickEvent`].
    */
    pub fn on_player_kick(
        &self,
        handler: impl Fn(&Server, &PlayerKickEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_kick>(handler);
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

impl Server {
    /**
    Calls `handler` when a player jumps: their client presses jump while they stand on the ground.

    Returning `true` changes nothing.

    See [`PlayerJumpEvent`].
    */
    pub fn on_player_jump(
        &self,
        handler: impl Fn(&Server, &PlayerJumpEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_jump>(handler);
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

impl Server {
    /**
    Calls `handler` when a player is about to get a potion effect, from a potion, food, a totem, a command or a plugin; `effect` is its name, such as `speed`.

    Returning `true` from a handler keeps the player from getting the effect.

    See [`PlayerEffectAddEvent`].
    */
    pub fn on_player_effect_add(
        &self,
        handler: impl Fn(&Server, &PlayerEffectAddEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_effect_add>(handler);
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

impl Server {
    /**
    Calls `handler` when a player loses a potion effect: it ran out, milk or a command cleared it, or a totem replaced it.

    Returning `true` changes nothing: the effect is removed anyway.

    See [`PlayerEffectRemoveEvent`].
    */
    pub fn on_player_effect_remove(
        &self,
        handler: impl Fn(&Server, &PlayerEffectRemoveEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_effect_remove>(handler);
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

impl Server {
    /**
    Calls `handler` when a player used a control of a data-driven screen: pressed a button, switched a toggle, moved a slider, chose an option or typed in a text field. The server has checked the value.

    Returning `true` from a handler puts the control back as it was, and keeps the handler of the screen from seeing it.

    See [`DduiResponseEvent`].
    */
    pub fn on_ddui_response(
        &self,
        handler: impl Fn(&Server, &DduiResponseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<ddui_response>(handler);
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

impl Server {
    /**
    Calls `handler` when a data-driven screen closed.

    Returning `true` changes nothing.

    See [`DduiCloseEvent`].
    */
    pub fn on_ddui_close(
        &self,
        handler: impl Fn(&Server, &DduiCloseEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<ddui_close>(handler);
    }
}
