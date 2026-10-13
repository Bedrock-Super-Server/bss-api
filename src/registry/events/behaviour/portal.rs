use crate::event::portal::{
    EndGatewayEvent, EntityPortalEvent, PlayerPortalEvent, PortalCreateEvent,
};
use crate::ffi::behaviour::{
    EndGatewayEventData, EntityPortalEventData, PlayerPortalEventData, PortalCreateEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    portal_create,
    PortalCreateEventData => PortalCreateEvent,
    |raw: &PortalCreateEventData| PortalCreateEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        width: raw.width,
        height: raw.height,
        axis: ffi_string(raw.axis),
        cause: ffi_string(raw.cause),
        player: ffi_string(raw.player),
        world: raw.world,
    }
);

impl Cancellable for PortalCreateEvent {}

impl Server {
    /**
    Calls `handler` when a portal is about to be created: a Nether portal is lit or generated for travel, or an End portal is completed.

    Returning `true` prevents the portal from being created.

    See [`PortalCreateEvent`].
    */
    pub fn on_portal_create(
        &self,
        handler: impl Fn(&Server, &PortalCreateEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<portal_create>(handler);
    }
}

hook!(
    player_portal,
    PlayerPortalEventData => PlayerPortalEvent,
    |raw: &PlayerPortalEventData| PlayerPortalEvent {
        player: ffi_string(raw.player),
        portal: ffi_string(raw.portal),
        from_world: raw.from_world,
        to_world: raw.to_world,
        x: std::cell::Cell::new(raw.x),
        y: std::cell::Cell::new(raw.y),
        z: std::cell::Cell::new(raw.z),
    },
    |safe: PlayerPortalEvent, raw: &mut PlayerPortalEventData| {
        raw.x = safe.x.get();
        raw.y = safe.y.get();
        raw.z = safe.z.get();
    }
);

impl Cancellable for PlayerPortalEvent {}

impl Server {
    /**
    Calls `handler` when a player goes through a portal.

    Returning `true` keeps the player where they are.

    See [`PlayerPortalEvent`].
    */
    pub fn on_player_portal(
        &self,
        handler: impl Fn(&Server, &PlayerPortalEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<player_portal>(handler);
    }
}

hook!(
    entity_portal,
    EntityPortalEventData => EntityPortalEvent,
    |raw: &EntityPortalEventData| EntityPortalEvent {
        entity: raw.entity,
        identifier: ffi_string(raw.identifier),
        portal: ffi_string(raw.portal),
        from_world: raw.from_world,
        to_world: raw.to_world,
        x: std::cell::Cell::new(raw.x),
        y: std::cell::Cell::new(raw.y),
        z: std::cell::Cell::new(raw.z),
    },
    |safe: EntityPortalEvent, raw: &mut EntityPortalEventData| {
        raw.x = safe.x.get();
        raw.y = safe.y.get();
        raw.z = safe.z.get();
    }
);

impl Cancellable for EntityPortalEvent {}

impl Server {
    /**
    Calls `handler` when an entity other than a player goes through a portal.

    Returning `true` keeps the entity where it is.

    See [`EntityPortalEvent`].
    */
    pub fn on_entity_portal(
        &self,
        handler: impl Fn(&Server, &EntityPortalEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<entity_portal>(handler);
    }
}

hook!(
    end_gateway,
    EndGatewayEventData => EndGatewayEvent,
    |raw: &EndGatewayEventData| EndGatewayEvent {
        x: raw.x,
        y: raw.y,
        z: raw.z,
        world: raw.world,
    }
);

impl Cancellable for EndGatewayEvent {}

impl Server {
    /**
    Calls `handler` when an End gateway is about to be generated after a dragon fight.

    Returning `true` prevents the gateway.

    See [`EndGatewayEvent`].
    */
    pub fn on_end_gateway(
        &self,
        handler: impl Fn(&Server, &EndGatewayEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<end_gateway>(handler);
    }
}
