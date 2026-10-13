//! The entry point of a plugin: the [`Plugin`] trait and [`export_plugin!`](crate::export_plugin).

use crate::server::Server;

/**
A plugin, as the server loads it.

The server calls [`register`](Plugin::register) once, when the plugin is loaded, and [`disable`](Plugin::disable) once, when it is unloaded or the server stops. Export the type with [`export_plugin!`](crate::export_plugin).

# Example

```
use bss_api::prelude::*;

struct Motd;

impl Plugin for Motd {
    fn name(&self) -> &'static str {
        "motd"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn register(&self, server: &Server) {
        server.register_event(Greeting);
    }

    fn disable(&self, server: &Server) {
        server.broadcast("§7The MOTD plugin is going away.");
    }
}

struct Greeting;

impl Event for Greeting {
    fn on_join(&self, server: &Server, event: &JoinEvent) {
        if let Some(player) = server.player(&event.name) {
            player.message("§aWelcome! Type /help to get started.");
        }
    }
}

export_plugin!(Motd);
```
*/
pub trait Plugin: Send + Sync {
    /// Name shown in the server log; plugins are told apart by it.
    fn name(&self) -> &'static str {
        "plugin"
    }

    /// Version shown in the server log, such as `env!("CARGO_PKG_VERSION")`.
    fn version(&self) -> &'static str {
        "0.0.0"
    }

    /// Called once when the plugin is loaded: register events, commands, content and tasks here.
    fn register(&self, server: &Server);

    /**
    Called once when the plugin is unloaded or the server stops.

    Event handlers, commands and scheduled tasks of the plugin are removed by the server; use this to save state or say goodbye.
    */
    fn disable(&self, _server: &Server) {}
}

/**
Exports a [`Plugin`] so the server can load the library.

Call it once, at the root of the crate, with an expression that builds the plugin. It defines the `bss_plugin_describe` symbol the server looks for, which tells the server the version of the plugin ABI the plugin was built with before anything else.

```
use bss_api::prelude::*;

struct Empty;

impl Plugin for Empty {
    fn register(&self, _server: &Server) {}
}

export_plugin!(Empty);
```
*/
#[macro_export]
macro_rules! export_plugin {
    ($plugin:expr) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn bss_plugin_describe() -> *const $crate::ffi::vtable::PluginVtable {
            $crate::registry::install(::std::boxed::Box::new($plugin));
            $crate::registry::describe()
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn plugin_panic_unwinds() -> bool {
            cfg!(panic = "unwind")
        }
    };
}
