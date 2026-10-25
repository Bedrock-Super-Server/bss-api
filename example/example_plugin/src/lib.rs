mod commands;
mod events;

use bss_api::prelude::{Plugin, Server, export_plugin};

use crate::commands::example1::Example1;
use crate::commands::example2::Example2;
use crate::events::chat::ChatFilter;
use crate::events::join::Welcome;

struct ExamplePlugin;

impl Plugin for ExamplePlugin {
    fn name(&self) -> &'static str {
        "example_plugin"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn register(&self, server: &Server) {
        server.register_event(Welcome);
        server.register_event(ChatFilter);
        server.register_command(Example1);
        server.register_command(Example2);
    }
}

export_plugin!(ExamplePlugin);
