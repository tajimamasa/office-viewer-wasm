# Office-Viewer-WASM

This project is a software that parses Office files.

> [!NOTE]
> This project is under development.

## Build

1. Build the WASM Part.

```bash
cd lib
wit-bindgen rust --out-dir src --world parser wit/world.wit
cargo component build --target wasm32-unknown-unknown
```

2. Build CLI Part.

```bash
cd cli
cargo run ../lib/target/wasm32-unknown-unknown/debug/office_viewer_lib.wasm
```
