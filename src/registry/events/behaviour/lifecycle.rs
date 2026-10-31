use crate::event::lifecycle::{
    ConsoleCommandEvent, PluginDisableEvent, PluginEnableEvent, ServerStartedEvent, ServerStopEvent,
};
use crate::ffi::behaviour::{
    ConsoleCommandEventData, PluginDisableEventData, PluginEnableEventData, ServerStartedEventData,
    ServerStopEventData,
};
use crate::registry::hook::{Cancellable, Verdict, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    server_started,
    ServerStartedEventData => ServerStartedEvent,
    |raw: &ServerStartedEventData| ServerStartedEvent {
        worlds: raw.worlds,
    }
);

impl Cancellable for ServerStartedEvent {}

impl ServerStartedEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_server_started<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ServerStartedEvent) -> V + Send + Sync + 'static,
    ) {
        register::<server_started, V>(handler);
    }
}

hook!(
    server_stop,
    ServerStopEventData => ServerStopEvent,
    |raw: &ServerStopEventData| ServerStopEvent {
        players: raw.players,
    }
);

impl Cancellable for ServerStopEvent {}

impl ServerStopEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_server_stop<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ServerStopEvent) -> V + Send + Sync + 'static,
    ) {
        register::<server_stop, V>(handler);
    }
}

thread_local! {
    static CONSOLE_COMMAND_COMMAND: std::cell::RefCell<String> = const { std::cell::RefCell::new(String::new()) };
}

hook!(
    console_command,
    ConsoleCommandEventData => ConsoleCommandEvent,
    |raw: &ConsoleCommandEventData| ConsoleCommandEvent {
        command: std::cell::RefCell::new(ffi_string(raw.command)),
    },
    |safe: ConsoleCommandEvent, raw: &mut ConsoleCommandEventData| {
        raw.command = crate::registry::hand_back(&CONSOLE_COMMAND_COMMAND, safe.command.into_inner());
    }
);

impl Cancellable for ConsoleCommandEvent {}

impl ConsoleCommandEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_console_command<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &ConsoleCommandEvent) -> V + Send + Sync + 'static,
    ) {
        register::<console_command, V>(handler);
    }
}

hook!(
    plugin_enable,
    PluginEnableEventData => PluginEnableEvent,
    |raw: &PluginEnableEventData| PluginEnableEvent {
        name: ffi_string(raw.name),
        version: ffi_string(raw.version),
    }
);

impl Cancellable for PluginEnableEvent {}

impl PluginEnableEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_plugin_enable<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PluginEnableEvent) -> V + Send + Sync + 'static,
    ) {
        register::<plugin_enable, V>(handler);
    }
}

hook!(
    plugin_disable,
    PluginDisableEventData => PluginDisableEvent,
    |raw: &PluginDisableEventData| PluginDisableEvent {
        name: ffi_string(raw.name),
        version: ffi_string(raw.version),
    }
);

impl Cancellable for PluginDisableEvent {}

impl PluginDisableEvent {
    /// Cancels the event: see its documentation for what that prevents.
    pub fn cancel(&self) {
        Cancellable::set_cancelled(self, true);
    }
}

impl Server {
    pub fn on_plugin_disable<V: Verdict>(
        &self,
        handler: impl Fn(&Server, &PluginDisableEvent) -> V + Send + Sync + 'static,
    ) {
        register::<plugin_disable, V>(handler);
    }
}
