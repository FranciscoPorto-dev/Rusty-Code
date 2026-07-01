# Rusty Code

A terminal-based AI coding agent written in Rust. **Rusty Code** is an open-source alternative to [OpenCode](https://opencode.ai) — built for developers who want a fast, native TUI experience with unrestricted access to LLM provider APIs.

## About

Rusty Code is a terminal user interface (TUI) coding assistant. It provides an interactive prompt where you can compose messages, navigate input with a cursor, and submit requests — all from your terminal.

The project is in early development. The current release focuses on the core TUI shell: a styled input area, editing modes, and keyboard-driven navigation. The goal is to grow into a full coding agent with multi-provider LLM support, session management, and project-aware tooling.

### Why Rusty Code instead of OpenCode?

[OpenCode](https://github.com/anomalyco/opencode) is a popular open-source AI coding agent with a TUI, desktop app, and IDE extension. Rusty Code aims to be a **Rust-native alternative** with a different design philosophy:

| | OpenCode | Rusty Code |
|---|---|---|
| **Runtime** | Node.js / TypeScript | Native Rust binary |
| **API access** | Routed through OpenCode's provider layer | Direct access to **all** supported LLM provider APIs |
| **Distribution** | npm, brew, install script | Single `cargo build` binary |
| **UI framework** | Custom TUI | [ratatui](https://ratatui.rs/) |

Unlike OpenCode, which manages providers through its own `/connect` flow and hosted auth, Rusty Code is being built so you can wire up **any API key and endpoint** for the providers you choose — Anthropic, OpenAI, Google, local models via Ollama, and others — without being limited to a curated provider list or proxy layer.

## Prerequisites

| Requirement | Version | Notes |
|---|---|---|
| [Rust](https://www.rust-lang.org/tools/install) | 1.85+ (edition 2024) | Install via `rustup` |
| Terminal emulator | Any modern TUI-capable terminal | e.g. Alacritty, WezTerm, Kitty, Ghostty |

## Dependencies

### Direct dependencies

These are declared in `Cargo.toml` and pulled from [crates.io](https://crates.io/):

| Crate | Version | Purpose |
|---|---|---|
| [ratatui](https://crates.io/crates/ratatui) | 0.30.2 | Terminal UI framework (widgets, layout, rendering, event loop via crossterm) |
| [color-eyre](https://crates.io/crates/color-eyre) | 0.6 | Colorized error reporting and panic hooks |
| [tui-big-text](https://crates.io/crates/tui-big-text) | 0.8 | Large pixel-style title text for the TUI header |

### Transitive dependencies

`ratatui`, `color-eyre`, and `tui-big-text` pull in additional crates automatically. The full resolved dependency tree (188 crates) is locked in `Cargo.lock`. Key transitive dependencies include:

| Crate | Purpose |
|---|---|
| `crossterm` | Cross-platform terminal manipulation (input, cursor, screen) |
| `mio` | Async I/O and event polling |
| `signal-hook` / `signal-hook-mio` | Unix signal handling for clean terminal restore |
| `unicode-width` / `unicode-segmentation` | Correct Unicode text width and grapheme handling |
| `compact_str` | Efficient string storage for TUI text |
| `hashbrown` | Hash map implementation used internally |
| `tracing` / `tracing-subscriber` | Structured logging (used by color-eyre) |
| `eyre` / `backtrace` | Error context and stack traces |
| `font8x8` | Bitmap font for big text rendering |
| `time` | Date/time formatting for widgets |
| `derive_builder` / `darling` | Proc-macro helpers for builder patterns |
| `libc` / `rustix` / `linux-raw-sys` | Low-level OS bindings |

To inspect the complete tree at any time:

```bash
cargo tree
```

## Getting started

### Clone and build

```bash
git clone <repository-url>
cd rusty_code
cargo build --release
```

### Run

```bash
cargo run
# or, after a release build:
./target/release/rusty_code
```

## Usage

| Key | Action |
|---|---|
| `e` | Enter editing mode |
| `Esc` | Return to normal mode |
| `Enter` | Submit the current input |
| `←` / `→` | Move cursor left / right |
| `Backspace` | Delete character before cursor |
| `q` | Quit |

## Project structure

```
src/
├── main.rs           # App loop, rendering, key handling
├── model.rs          # App state and input modes
├── view.rs           # Layout helpers
└── controller/
    ├── cursor.rs     # Cursor movement
    └── input.rs      # Character input, deletion, submit
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Format
cargo fmt

# Lint
cargo clippy
```

CI runs `cargo build` and `cargo test` on every push and pull request to `main` (see `.github/workflows/rust.yml`).

## License

Licensed under the [Apache License, Version 2.0](LICENSE).
