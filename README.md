# log-parser

A high-performance client-side log parser built with Rust and WebAssembly. Streams raw log file bytes directly into Wasm linear memory, indexes line offsets, and enables instant searching, filtering, and pagination of multi-gigabyte log files in the browser without server round-trips.

## What it does

Instead of uploading log files to a server or converting them into heavy JavaScript strings, this tool processes everything locally in the browser:

- **Zero-copy indexing** — raw file bytes are passed directly to Wasm, which builds a byte-offset index of all lines
- **Instant access** — any line can be retrieved in O(1) by index
- **Log level detection** — each line is classified as ERROR, WARN, INFO, DEBUG, or OTHER (case-insensitive)
- **Search with highlighting** — text search across all lines with match highlighting
- **Level filtering** — toggle visibility of specific log levels with match counts
- **Pagination** — pages through results without rendering the entire file to the DOM

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (Edition 2024 support required)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/): `cargo install wasm-pack`
- A static file server (Python, Node, etc.)

## Setup

```sh
git clone https://github.com/arit-pal/log-parser
cd log-parser
wasm-pack build --target web
```

## Running

```sh
npx serve .
```

Open [http://localhost:3000/ui/](http://localhost:3000/ui/) in your browser. Upload any log file to start exploring it.

> **Note:** You cannot use `file://` — Wasm modules require HTTP to load.

## Project structure

```
log-parser/
├── Cargo.toml         # Rust crate config, wasm-bindgen dependency
├── src/
│   └── lib.rs         # Core Rust indexer (LogIndexer struct)
├── pkg/               # wasm-pack output (committed, regenerated on build)
│   ├── log_parser.js
│   └── log_parser_bg.wasm
└── ui/
    └── index.html     # Vanilla HTML/JS frontend
```

## How it works

1. User uploads a log file via the browser
2. JavaScript reads the file as an `ArrayBuffer` and passes the raw bytes to the Rust `LogIndexer` constructor
3. Wasm scans the bytes once, recording the start offset of each line and detecting its log level
4. The UI calls `get_line(i)` and `get_line_level(i)` to render pages of results
5. Search and level filters operate on the indexed data — no re-parsing needed
