# example_plugin

A small plugin for Bedrock Super Server, built with `bss-api`

## Build

install the WebAssembly target:

```sh
rustup target add wasm32-wasip1
```

then :

```sh
cargo build --release --target wasm32-wasip1
```

The plugin is `target/wasm32-wasip1/release/example_plugin.wasm`. Copy it into the `plugins/` folder of the server and restart the server
