/*!
The plugin API of [Bedrock Super Server](https://github.com/cmgcpf/Netherite).

A plugin is a native library the server loads from its `plugins/` folder. It reacts to events, adds commands, forms, items and blocks, and drives players and worlds through a [`Server`](server::Server) handle.

# Quick start

Create a library crate that builds a `cdylib`:

```toml
[package]
name = "greeter"
version = "1.0.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
bss-api = "1"
```

Then implement [`Plugin`](sdk::Plugin) and export it with [`export_plugin!`]:

```
use bss_api::prelude::*;

struct Greeter;

impl Plugin for Greeter {
    fn name(&self) -> &'static str {
        "greeter"
    }

    fn version(&self) -> &'static str {
        "1.0.0"
    }

    fn register(&self, server: &Server) {
        server.register_event(Welcome);
        server.register_command(Hello);
    }
}

/// Greets every player who joins.
struct Welcome;

impl Event for Welcome {
    fn on_join(&self, _server: &Server, event: &JoinEvent) {
        event.set_message(&format!("§e{} joined the game", event.name));
    }
}

/// `/hello`, which answers the player who runs it.
struct Hello;

impl Command for Hello {
    fn name(&self) -> &'static str {
        "hello"
    }

    fn description(&self) -> &'static str {
        "Say hello"
    }

    fn execute(&self, ctx: &CommandContext) -> CommandResult {
        ctx.reply(&format!("Hello, {}!", ctx.sender()));
        Ok(())
    }
}

export_plugin!(Greeter);
```

Build it with `cargo build --release` and copy the library from `target/release/` (`libgreeter.so` on Linux, `greeter.dll` on Windows) into the `plugins/` folder of the server, then restart it.

# Where to start

| To... | Use |
|---|---|
| react to the game | [`Event`](prelude::Event) with [`Server::register_event`](server::Server::register_event), or a `Server::on_*` method; the types live in [`event`] |
| add a command | [`Command`](command::Command) with [`Server::register_command`](server::Server::register_command) |
| talk to a player, show forms, open inventories | [`Player`](player::Player), [`player::form`] |
| run code later or repeatedly | [`Server::schedule`](server::Server::schedule) |
| read and change worlds | [`Server`](server::Server), [`server::world`] |
| add items, blocks and recipes | [`content`] |
| apply potion effects | [`server::effect`] |
| send any packet, by name with JSON fields | [`Player::send_packet`](player::Player::send_packet), [`Server::broadcast_packet`](server::Server::broadcast_packet), [`PacketEvent::fields`](event::PacketEvent::fields) |

[`prelude`] brings the common names into scope with one `use`.

# Threads and panics

The server calls handlers from several threads, which is why they must be `Send + Sync`. A panic in a handler is caught and logged: the server keeps running and other handlers still run. A plugin must therefore be built with `panic = "unwind"`, the default; the server refuses a plugin built with `panic = "abort"`.

# Compatibility

A plugin talks to the server through tables of C functions whose version, [`ABI_MAJOR`](ffi::ABI_MAJOR)`.`[`ABI_MINOR`](ffi::ABI_MINOR), the server reads before anything else. It refuses a plugin of another major version and loads one of any minor version: every table starts with its size, so what one side added since the other was built reads as absent instead of being read past its end.
*/

#![warn(missing_docs)]
#![allow(clippy::type_complexity)]

#[doc(hidden)]
#[macro_use]
pub mod ffi;

pub mod command;
pub mod content;
pub mod event;
pub mod math;
pub mod player;
#[doc(hidden)]
pub mod registry;
pub mod sdk;
pub mod server;

pub use ffi::abi::{HeldItem, ItemStack};

/// The `serde_json` crate this API takes and returns JSON with, for
/// [`Player::send_packet`](player::Player::send_packet) and the rest. Use it, or its [`json!`](serde_json::json)
/// macro from the prelude, rather than a `serde_json` of your own: a plugin that depends on
/// another version of `serde_json` has another `Value` type, which these methods do not accept.
pub use serde_json;

/// The names most plugins need, in one `use bss_api::prelude::*;`.
pub mod prelude {
    pub use crate::command::{Command, CommandContext, CommandResult, Param, SubCommand};
    pub use crate::content::{
        ArmorSlot, Category, CustomBlock, CustomItem, GroupDef, Material, RecipeDef, ToolType,
    };
    pub use crate::event::*;
    pub use crate::export_plugin;
    pub use crate::ffi::abi::ItemStack;
    pub use crate::math::BlockFace;
    pub use crate::player::Player;
    pub use crate::player::client_data::ClientData;
    pub use crate::player::form::{
        Button, CustomForm, Element, Form, FormResponse, FormValue, Image, ModalForm, SimpleForm,
    };
    pub use crate::player::inventory::{Inventory, VirtualItem};
    pub use crate::player::jwt::{AuthInfo, ClientDataJwt, Jwt, JwtClaims, JwtHeader, LoginJwt};
    pub use crate::player::uuid::{Uuid, Variant, Version};
    pub use crate::registry::events::handler::Event;
    pub use crate::registry::hook::{Cancellable, EventOptions, Priority};
    pub use crate::sdk::Plugin;
    pub use crate::server::catalog::{particle, sound};
    pub use crate::server::effect::Effect;
    pub use crate::server::entity::{ActorFlag, Entity};
    pub use crate::server::scheduler::TaskId;
    pub use crate::server::world::{Block, Chunk, Pos, Weather, World};
    pub use crate::server::{Dimension, PacketInfo, PluginInfo, PluginUsage, Server};
    pub use serde_json::{Value, json};
}
