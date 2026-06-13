# log-parser

Rust + WebAssembly log indexer. Compiles `src/lib.rs` to wasm via `wasm-bindgen`, producing JS/Wasm output in `pkg/`. The UI demo (`ui/index.html`) imports directly from `../pkg/log_parser.js`.

## Build

```sh
# Requires: rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/log_parser.wasm --out-dir pkg --target web
```

## Verify the UI

Open `ui/index.html` in a browser (requires a local HTTP server, not `file://` due to Wasm loading).

## Key facts

- **Edition 2024** — requires recent nightly or stable Rust that supports it.
- **Single source file**: `src/lib.rs` — all logic is in `LogIndexer` (constructor indexes line offsets and scans for `ERROR`).
- **`pkg/`** is the wasm-bindgen output directory (committed). Edit `src/lib.rs`, rebuild, and `pkg/` updates.
- **No tests, linter, formatter, or CI** currently configured.
- The `LogIndexer` constructor accepts `Vec<u8>` (raw file bytes), builds line-offset index, and flags lines containing `ERROR`.
