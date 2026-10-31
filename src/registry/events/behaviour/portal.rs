use crate::event::portal::{
    EndGatewayEvent, EntityPortalEvent, PlayerPortalEvent, PortalCreateEvent,
};
use crate::ffi::behaviour::{
    EndGatewayEventData, EntityPortalEventData, PlayerPortalEventData, PortalCreateEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
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

impl PortalCreateEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_portal_create<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PortalCreateEvent) -> V + Send + Sync + 'static,
    ) {
        register::<portal_create, V>(handler);
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

impl PlayerPortalEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_player_portal<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PlayerPortalEvent) -> V + Send + Sync + 'static,
    ) {
        register::<player_portal, V>(handler);
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

impl EntityPortalEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_entity_portal<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EntityPortalEvent) -> V + Send + Sync + 'static,
    ) {
        register::<entity_portal, V>(handler);
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

impl EndGatewayEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_end_gateway<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &EndGatewayEvent) -> V + Send + Sync + 'static,
    ) {
        register::<end_gateway, V>(handler);
    }
}
