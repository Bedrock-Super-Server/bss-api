# bss-api - 1.4

The plugin API of Bedrock Super Server, a software for Minecraft Bedrock Edition.

A plugin is a native library that the server loads from its `plugins/` folder. With this crate it can:

- react to more than a hundred game events, and cancel or change most of them;
- add commands with subcommands, parameters and permissions;
- show forms and virtual chest menus to players;
- add custom items, blocks, recipes and creative inventory groups;
- read and change worlds, generate chunks, control the weather;
- add its own entities, apply potion effects, schedule tasks;
- offer services that other plugins call.

## Quick start

```toml
[package]
name = "salut"
version = "1.0.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
bss-api = { git = "https://github.com/Bedrock-Super-Server/bss-api" }
```

Cargo downloads the API from the server's repository on the first build. `cargo update -p bss-api` fetches its latest version.

```rust
use bss_api::prelude::*;

struct Salut;

impl Plugin for Salut {
    fn name(&self) -> &'static str {
        "salut"
    }

    fn version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn register(&self, server: &Server) {
        server.register_event(Welcome);
    }
}

struct Welcome;

impl Event for Welcome {
    fn on_join(&self, _server: &Server, event: &JoinEvent) {
        event.set_message(&format!("§e{} joined the game", event.name));
    }
}

export_plugin!(Salut);
```

Build with `cargo build --release`, copy the library from `target/release/` (`salut.dll` on Windows, `libsalut.so` on Linux) into the `plugins/` folder of the server, and restart it.

The crate documentation, `cargo doc --open`, covers every type, with an example for each part of the API.

## Compatibility

| bss-api | Bedrock Super Server | Minecraft |
|---|---|---|
| 1.x | 2.1 | Bedrock 1.26.52 |

A plugin and the server share a versioned ABI, `major.minor`, and the major version of `bss-api` is the major version of the ABI. The server reads the version a plugin was built with before anything else, refuses another major version with a message in its console, and loads a plugin of any minor version: what one side added since the other was built reads as absent. A plugin built with `bss-api` 1.x therefore keeps working on every server of ABI 1, older or newer, without being rebuilt.

## Minimum Rust version

Rust 1.88.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Plugins built with this crate can use any license.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
