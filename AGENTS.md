# log-parser

Rust + WebAssembly log indexer. Compiles `src/lib.rs` to wasm via `wasm-pack`, producing JS/Wasm output in `pkg/`. The UI demo (`ui/index.html`) imports directly from `../pkg/log_parser.js`.

## Build

```sh
# Requires: rustup target add wasm32-unknown-unknown
# Requires: cargo install wasm-pack
wasm-pack build --target web
```

## Run the UI

```sh
npx serve .
# Open http://localhost:3000/ui/
```

Cannot use `file://` due to Wasm module loading restrictions.

## Key facts

- **Edition 2024** — requires recent nightly or stable Rust that supports it.
- **Single source file**: `src/lib.rs` — all logic is in `LogIndexer`.
- **`pkg/`** is the wasm-pack output directory (committed). Edit `src/lib.rs`, rebuild, and `pkg/` updates.
- **No tests, linter, formatter, or CI** currently configured.
- **`ui/index.html`** — vanilla HTML/JS, no framework or bundler.

## WASM API (`LogIndexer`)

| Method | Returns | Description |
|--------|---------|-------------|
| `new(data: Vec<u8>)` | constructor | Indexes line offsets, detects levels and timestamps |
| `total_lines()` | `usize` | Total number of lines |
| `total_errors()` | `usize` | Lines containing ERROR |
| `get_line(i)` | `Option<String>` | Line content at index (UTF-8, \r stripped) |
| `get_line_level(i)` | `u8` | Level: 0=NONE, 1=DEBUG, 2=INFO, 3=WARN, 4=ERROR |
| `get_line_timestamp(i)` | `f64` | Unix epoch seconds, or -1.0 if no timestamp detected |
| `count_by_level(level)` | `usize` | Count of lines at a given level (precomputed) |

## Level detection

Word-boundary, case-insensitive matching against: `ERROR`/`ERRO`, `WARN`, `INFO`, `DEBUG`/`DBG`. Priority: ERROR > WARN > INFO > DEBUG > NONE. Handles `\r\n` Windows line endings and trailing lines without a newline terminator.

## Timestamp detection

Parses the first bytes of each line for:
- **ISO 8601**: `2025-01-15T10:30:00Z`, `2025-01-15 10:30:00+05:30`
- **Common log format**: `15/Jan/2025:10:30:00 +0000`
- **Unix timestamps**: 10-digit (seconds) or 13-digit (milliseconds)

Returns epoch seconds as `f64`, or -1.0 if no timestamp detected.
