# bss-api 2.0

The plugin API of Bedrock Super Server

A plugin is a WebAssembly module, one `.wasm` file for every system and processor, that the server loads from its `plugins/` folder and runs in a sandbox. With this crate it can:

## Compatibility

| bss-api | Bedrock Super Server | Minecraft |
|---|---|---|
| 2.0 | 0.2.42 | Bedrock 1.26.60 |
| 1.10 | 0.2.30 | Bedrock 1.26.52 |
| 1.8 | 0.2.24 | Bedrock 1.26.52 |

A plugin and the server share a versioned ABI, `major.minor`, and the major version of `bss-api` is the major version of the ABI. The server reads the version a plugin was built with before anything else, refuses another major version with a message in its console, and loads a plugin of any minor version: what one side added since the other was built reads as absent. A plugin built with `bss-api` 1.x therefore keeps working on every server of ABI 1, older or newer, without being rebuilt.

## Minimum Rust version

Rust 1.88.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. Plugins built with this crate can use any license.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
