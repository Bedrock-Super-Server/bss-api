use std::cell::Cell;

use crate::event::player::{
    AnimationEvent, ArmorEquipEvent, BucketUseEvent, ChatEvent, CommandEvent, ConsumeEvent,
    DeathEvent, DropItemEvent, EmoteEvent, FormResponseEvent, GameModeChangeEvent, InteractEvent,
    ItemHeldEvent, ItemUseEvent, JoinEvent, LoginEvent, MoveEvent, PickupItemEvent, QuitEvent,
    RespawnEvent, SkinChangeEvent, TeleportEvent, ToggleEvent, ToggleKind,
};
use crate::ffi::events::*;
use crate::player::client_data::ClientData;
use crate::player::jwt::LoginJwt;
use crate::registry::host_ptr::host_bytes;
use crate::registry::macros::{
    event_hook, event_hook_cancel, event_hook_cancel_message, event_hook_message,
};
use crate::server::{Server, ffi_string};

event_hook_message!(
    on_join,
    JOIN,
    join,
    JOIN_MESSAGE,
    PlayerJoinEventData,
    JoinEvent
);
event_hook_message!(
    on_quit,
    QUIT,
    quit,
    QUIT_MESSAGE,
    PlayerQuitEventData,
    QuitEvent
);

event_hook!(
    on_login,
    LOGIN,
    login,
    PlayerLoginEventData,
    LoginEvent,
    |e: &PlayerLoginEventData| {
        let jwt = LoginJwt::from_json(&ffi_string(e.login_json));
        LoginEvent {
            name: ffi_string(e.name),
            xuid: ffi_string(e.xuid),
            uuid: ffi_string(e.uuid),
            ip: ffi_string(e.ip),
            authenticated: e.authenticated,
            client_data: ClientData::from_json(&ffi_string(e.client_data_json)),
            protocol: jwt.protocol,
            negotiated_protocol: jwt.negotiated_protocol,
            auth_info: jwt.auth_info,
            token: jwt.token,
            client_data_token: jwt.client_data_token,
        }
    }
);

event_hook_cancel_message!(
    on_chat,
    CHAT,
    chat,
    CHAT_MESSAGE,
    PlayerChatEventData,
    ChatEvent,
    |e: &PlayerChatEventData| ChatEvent {
        name: ffi_string(e.name),
        source_name: ffi_string(e.source_name),
        message: ffi_string(e.message),
        message_type: e.message_type,
        xbox_user_id: ffi_string(e.xbox_user_id),
        platform_chat_id: ffi_string(e.platform_chat_id),
        formatted: std::cell::RefCell::new(ffi_string(e.formatted)),
    },
    formatted
);

event_hook_cancel!(
    on_move,
    MOVE,
    movement,
    PlayerMoveEventData,
    MoveEvent,
    |e: &PlayerMoveEventData| MoveEvent {
        name: ffi_string(e.name),
        from: (e.from_x, e.from_y, e.from_z),
        to: (e.to_x, e.to_y, e.to_z),
        pitch: e.pitch,
        yaw: e.yaw,
        sneaking: e.sneaking,
        sprinting: e.sprinting,
    }
);

event_hook_cancel!(
    on_command,
    COMMAND_EVENT,
    command,
    PlayerCommandEventData,
    CommandEvent,
    |e: &PlayerCommandEventData| CommandEvent {
        name: ffi_string(e.name),
        command: ffi_string(e.command),
    }
);

event_hook_cancel!(
    on_interact,
    INTERACT,
    interact,
    PlayerInteractEventData,
    InteractEvent,
    |e: &PlayerInteractEventData| InteractEvent {
        name: ffi_string(e.name),
        action: e.action,
        x: e.x,
        y: e.y,
        z: e.z,
        face: e.face,
        item_id: e.item_id,
        block_runtime_id: e.block_runtime_id,
        hotbar_slot: e.hotbar_slot,
        click_x: e.click_x,
        click_y: e.click_y,
        click_z: e.click_z,
    }
);

event_hook_cancel!(
    on_animation,
    ANIMATION,
    animation,
    PlayerAnimationEventData,
    AnimationEvent,
    |e: &PlayerAnimationEventData| AnimationEvent {
        name: ffi_string(e.name),
        action: e.action,
        data: e.data,
    }
);

event_hook_cancel!(
    on_item_held,
    ITEM_HELD,
    item_held,
    PlayerItemHeldEventData,
    ItemHeldEvent,
    |e: &PlayerItemHeldEventData| ItemHeldEvent {
        name: ffi_string(e.name),
        old_slot: e.old_slot,
        new_slot: e.new_slot,
        item_id: e.item_id,
    }
);

event_hook_cancel!(
    on_drop_item,
    DROP_ITEM,
    drop_item,
    PlayerDropItemEventData,
    DropItemEvent,
    |e: &PlayerDropItemEventData| DropItemEvent {
        name: ffi_string(e.name),
        item_id: e.item_id,
        block_runtime_id: e.block_runtime_id,
        count: e.count,
        source_slot: e.source_slot,
    }
);

event_hook_cancel!(
    on_pickup_item,
    PICKUP_ITEM,
    pickup_item,
    PlayerPickupItemEventData,
    PickupItemEvent,
    |e: &PlayerPickupItemEventData| PickupItemEvent {
        name: ffi_string(e.name),
        item_id: e.item_id,
        block_runtime_id: e.block_runtime_id,
        count: e.count,
        entity_id: e.entity_id,
    }
);

event_hook_cancel!(
    on_gamemode_change,
    GAMEMODE_CHANGE,
    gamemode_change,
    PlayerGameModeChangeEventData,
    GameModeChangeEvent,
    |e: &PlayerGameModeChangeEventData| {
        GameModeChangeEvent {
            name: ffi_string(e.name),
            from: e.from,
            to: e.to,
        }
    }
);

event_hook_cancel!(
    on_consume,
    CONSUME,
    consume,
    PlayerConsumeEventData,
    ConsumeEvent,
    |e: &PlayerConsumeEventData| ConsumeEvent {
        name: ffi_string(e.name),
        item_id: e.item_id,
        food_points: Cell::new(e.food_points),
        saturation: Cell::new(e.saturation),
    },
    [food_points, saturation]
);

event_hook_cancel!(
    on_teleport,
    TELEPORT,
    teleport,
    PlayerTeleportEventData,
    TeleportEvent,
    |e: &PlayerTeleportEventData| TeleportEvent {
        name: ffi_string(e.name),
        from: (e.from_x, e.from_y, e.from_z),
        to: (e.to_x, e.to_y, e.to_z),
    }
);

thread_local! {
    static DEATH_MESSAGE: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

crate::registry::hook::hook!(
    death,
    PlayerDeathEventData => DeathEvent,
    |raw: &PlayerDeathEventData| DeathEvent {
        name: ffi_string(raw.name),
        killer: ffi_string(raw.killer),
        message: std::cell::RefCell::new(ffi_string(raw.message)),
        keep_inventory: std::cell::Cell::new(raw.keep_inventory),
    },
    |safe: DeathEvent, raw: &mut PlayerDeathEventData| {
        raw.message = crate::registry::hand_back(&DEATH_MESSAGE, safe.message.into_inner());
        raw.keep_inventory = safe.keep_inventory.get();
    }
);

crate::registry::hook::hook!(
    respawn,
    PlayerRespawnEventData => RespawnEvent,
    |raw: &PlayerRespawnEventData| RespawnEvent {
        name: ffi_string(raw.name),
        x: std::cell::Cell::new(raw.x),
        y: std::cell::Cell::new(raw.y),
        z: std::cell::Cell::new(raw.z),
    },
    |safe: RespawnEvent, raw: &mut PlayerRespawnEventData| {
        raw.x = safe.x.get();
        raw.y = safe.y.get();
        raw.z = safe.z.get();
    }
);

impl Server {
    /**
    Calls `handler` when a player dies.

    See [`DeathEvent`](crate::event::DeathEvent).
    */
    pub(crate) fn on_death(&self, handler: impl Fn(&Server, &DeathEvent) + Send + Sync + 'static) {
        crate::registry::hook::register::<death, _>(move |server, event| {
            handler(server, event);
            false
        });
    }

    /**
    Calls `handler` when a player respawns after death.

    See [`RespawnEvent`](crate::event::RespawnEvent).
    */
    pub(crate) fn on_respawn(
        &self,
        handler: impl Fn(&Server, &RespawnEvent) + Send + Sync + 'static,
    ) {
        crate::registry::hook::register::<respawn, _>(move |server, event| {
            handler(server, event);
            false
        });
    }
}

event_hook_cancel!(
    on_item_use,
    ITEM_USE,
    item_use,
    ItemUseEventData,
    ItemUseEvent,
    |e: &ItemUseEventData| ItemUseEvent {
        name: ffi_string(e.name),
        item_id: e.item_id,
        block_runtime_id: e.block_runtime_id,
        action: e.action,
        x: e.x,
        y: e.y,
        z: e.z,
        face: e.face,
        hotbar_slot: e.hotbar_slot,
        click_x: e.click_x,
        click_y: e.click_y,
        click_z: e.click_z,
    }
);

event_hook!(
    on_toggle,
    TOGGLE,
    toggle,
    PlayerToggleEventData,
    ToggleEvent,
    |e: &PlayerToggleEventData| ToggleEvent {
        name: ffi_string(e.name),
        kind: ToggleKind::from_raw(e.kind),
        active: e.active,
    }
);

event_hook!(
    on_bucket_use,
    BUCKET_USE,
    bucket_use,
    BucketUseEventData,
    BucketUseEvent,
    |e: &BucketUseEventData| BucketUseEvent {
        name: ffi_string(e.name),
        item_id: e.item_id,
        x: e.x,
        y: e.y,
        z: e.z,
        filling: e.filling,
    }
);

event_hook_cancel_message!(
    on_emote,
    EMOTE,
    emote,
    EMOTE_ID,
    PlayerEmoteEventData,
    EmoteEvent,
    |e: &PlayerEmoteEventData| EmoteEvent {
        name: ffi_string(e.name),
        entity_id: e.entity_id,
        duration: e.duration,
        xuid: ffi_string(e.xuid),
        platform_id: ffi_string(e.platform_id),
        flags: e.flags,
        emote_id: std::cell::RefCell::new(ffi_string(e.emote_id)),
    },
    emote_id
);

event_hook_cancel!(
    on_skin_change,
    SKIN_CHANGE,
    skin_change,
    PlayerSkinChangeEventData,
    SkinChangeEvent,
    |e: &PlayerSkinChangeEventData| SkinChangeEvent {
        name: ffi_string(e.name),
        skin: host_bytes(e.skin.ptr, e.skin.len),
        new_skin_name: ffi_string(e.new_skin_name),
        old_skin_name: ffi_string(e.old_skin_name),
    }
);

event_hook_cancel!(
    on_form_response,
    FORM_RESPONSE,
    form_response,
    FormResponseEventData,
    FormResponseEvent,
    |e: &FormResponseEventData| FormResponseEvent {
        name: ffi_string(e.name),
        form_id: e.form_id,
        response: (!e.closed).then(|| ffi_string(e.response)),
    }
);

event_hook_cancel!(
    on_armor_equip,
    ARMOR_EQUIP,
    armor_equip,
    ArmorEquipEventData,
    ArmorEquipEvent,
    |e: &ArmorEquipEventData| ArmorEquipEvent {
        name: ffi_string(e.name),
        item_id: e.item_id,
        count: e.count,
        block_runtime_id: e.block_runtime_id,
        source_slot: e.source_slot,
        armor_slot: e.armor_slot,
    }
);
