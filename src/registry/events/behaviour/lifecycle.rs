use crate::event::lifecycle::{
    ConsoleCommandEvent, PluginDisableEvent, PluginEnableEvent, ServerStartedEvent, ServerStopEvent,
};
use crate::ffi::behaviour::{
    ConsoleCommandEventData, PluginDisableEventData, PluginEnableEventData, ServerStartedEventData,
    ServerStopEventData,
};
use crate::registry::hook::{Cancellable, hook, register};
use crate::server::{Server, ffi_string};

hook!(
    server_started,
    ServerStartedEventData => ServerStartedEvent,
    |raw: &ServerStartedEventData| ServerStartedEvent {
        worlds: raw.worlds,
    }
);

impl Cancellable for ServerStartedEvent {}

impl Server {
    /**
    Calls `handler` when the server finished starting and now accepts players. `worlds` is how many worlds are loaded.

    Returning `true` changes nothing.

    See [`ServerStartedEvent`].
    */
    pub fn on_server_started(
        &self,
        handler: impl Fn(&Server, &ServerStartedEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<server_started>(handler);
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

impl Server {
    /**
    Calls `handler` when the server begins to stop, before it disconnects the players, unloads the plugins and saves the worlds. `players` is how many are online.

    Returning `true` does not keep the server from stopping.

    See [`ServerStopEvent`].
    */
    pub fn on_server_stop(
        &self,
        handler: impl Fn(&Server, &ServerStopEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<server_stop>(handler);
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

impl Server {
    /**
    Calls `handler` when a line is typed in the server console, before it runs. Change `command` to run something else.

    Returning `true` from a handler keeps the line from running.

    See [`ConsoleCommandEvent`].
    */
    pub fn on_console_command(
        &self,
        handler: impl Fn(&Server, &ConsoleCommandEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<console_command>(handler);
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

impl Server {
    /**
    Calls `handler` when a plugin finished loading and is now enabled, itself included.

    Returning `true` changes nothing.

    See [`PluginEnableEvent`].
    */
    pub fn on_plugin_enable(
        &self,
        handler: impl Fn(&Server, &PluginEnableEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<plugin_enable>(handler);
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

impl Server {
    /**
    Calls `handler` when a plugin is about to be disabled, by a reload, an unload or the server stopping.

    Returning `true` does not keep the plugin loaded.

    See [`PluginDisableEvent`].
    */
    pub fn on_plugin_disable(
        &self,
        handler: impl Fn(&Server, &PluginDisableEvent) -> bool + Send + Sync + 'static,
    ) {
        register::<plugin_disable>(handler);
    }
}
