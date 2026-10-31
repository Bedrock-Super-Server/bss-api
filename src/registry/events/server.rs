use crate::event::server::{PacketEvent, ServerSettingsRequestEvent, TickEvent};
use crate::ffi::events::{
    ClientBoundEventData, ServerBoundEventData, ServerSettingsRequestEventData, ServerTickEventData,
};
use crate::registry::host_ptr::host_bytes;
use crate::server::{Server, ffi_string};

crate::registry::hook::hook!(
    tick,
    ServerTickEventData => TickEvent,
    |raw: &ServerTickEventData| TickEvent { tick: raw.tick }
);

impl Server {
    /**
    Calls `handler` when the server runs a game tick, twenty times per second.

    See [`TickEvent`](crate::event::TickEvent).
    */
    pub(crate) fn on_tick(&self, handler: impl Fn(&Server, &TickEvent) + Send + Sync + 'static) {
        crate::registry::hook::register::<tick, _>(move |server, event| {
            handler(server, event);
            false
        });
    }
}

/// Subscribes to the tick once, to run the plugin's scheduled tasks, the first time one is scheduled.
pub(crate) fn run_scheduled_tasks(server: &Server) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        server.with_event_options(crate::registry::hook::EventOptions::default(), |_| {
            crate::registry::hook::register::<tick, _>(|server, event| {
                crate::server::scheduler::run_due(server, event.tick);
                false
            });
        });
    });
}

crate::registry::hook::hook!(
    server_bound,
    ServerBoundEventData => PacketEvent,
    |raw: &ServerBoundEventData| PacketEvent {
        entity_id: raw.entity_id,
        name: ffi_string(raw.name),
        packet_id: raw.packet_id,
        data: host_bytes(raw.data.ptr, raw.data.len),
        player: ffi_string(raw.player),
    }
);

crate::registry::hook::hook!(
    client_bound,
    ClientBoundEventData => PacketEvent,
    |raw: &ClientBoundEventData| PacketEvent {
        entity_id: raw.entity_id,
        name: ffi_string(raw.name),
        packet_id: raw.packet_id,
        data: host_bytes(raw.data.ptr, raw.data.len),
        player: ffi_string(raw.player),
    }
);

impl crate::registry::hook::Cancellable for PacketEvent {}

impl PacketEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        crate::registry::hook::Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    /**
    Calls `handler` when a game packet passes from a player to the server.

    `event.cancel()` drops the packet.

    See [`PacketEvent`](crate::event::PacketEvent).
    */
    pub(crate) fn on_server_bound(
        &self,
        handler: impl Fn(&Server, &PacketEvent) -> bool + Send + Sync + 'static,
    ) {
        crate::registry::hook::register::<server_bound, _>(handler);
    }

    /**
    Calls `handler` when a game packet passes from the server to a player.

    `event.cancel()` drops the packet.

    See [`PacketEvent`](crate::event::PacketEvent).
    */
    pub(crate) fn on_client_bound(
        &self,
        handler: impl Fn(&Server, &PacketEvent) -> bool + Send + Sync + 'static,
    ) {
        crate::registry::hook::register::<client_bound, _>(handler);
    }
}

crate::registry::hook::hook!(
    server_settings_request,
    ServerSettingsRequestEventData => ServerSettingsRequestEvent,
    |raw: &ServerSettingsRequestEventData| ServerSettingsRequestEvent {
        name: ffi_string(raw.name),
    }
);

impl Server {
    /**
    Calls `handler` when a player opens the server settings page of their game settings.

    See [`ServerSettingsRequestEvent`](crate::event::ServerSettingsRequestEvent).
    */
    pub(crate) fn on_server_settings_request(
        &self,
        handler: impl Fn(&Server, &ServerSettingsRequestEvent) + Send + Sync + 'static,
    ) {
        crate::registry::hook::register::<server_settings_request, _>(move |server, event| {
            handler(server, event);
            false
        });
    }
}
