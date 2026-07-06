# Rusty Code

**Rusty Code** is an open-source, terminal-based AI coding agent written in Rust. It provides an interactive TUI (Terminal User Interface) for chatting with large language models, navigating your codebase, and getting help with coding tasks — all from the command line.

## About This Project

Rusty Code is built as a **Rust-native alternative to [OpenCode](https://opencode.ai/)**. Like OpenCode, it aims to bring AI-assisted development into your terminal with a fast, keyboard-driven interface. Where OpenCode is model-neutral and supports many providers, Rusty Code is designed to go further by offering **access to all major LLM APIs** — OpenAI, Anthropic, Google Gemini, AWS Bedrock, Groq, Azure OpenAI, OpenRouter, local models via Ollama, and more — without locking you into a single vendor.

The project is in early development. The current TUI foundation includes:

- A centered, wrapping input area with normal and editing modes
- Keyboard-driven navigation (cursor movement, submit, quit)
- Editing shortcuts — undo, clear, paste, delete-to-line-start, and move-to-start
- Clipboard paste via `arboard` (Ctrl+V / Cmd+V on macOS)
- Unicode-aware cursor positioning for multibyte input
- A styled title banner and context-sensitive help text
- A modular architecture split into **model**, **view**, and **controller** layers

Planned capabilities include multi-provider API integration, session management, tool use (file search, shell commands, code edits), and persistent conversation history.

## Why Rusty Code?

| Feature | OpenCode | Rusty Code |
|---|---|---|
| Language | Go | **Rust** |
| Terminal UI | Bubble Tea | **Ratatui** |
| Model providers | 75+ | **All major APIs** (planned) |
| Performance | Fast | **Native, zero-cost abstractions** |
| Binary size | Moderate | **Small, statically linked** |

If you want a lightweight, Rust-powered coding agent that can talk to any LLM provider you configure, Rusty Code is for you.

## Prerequisites

### System Requirements

- A modern terminal emulator (e.g. [WezTerm](https://wezfurlong.org/wezterm/), [Alacritty](https://alacritty.org/), [Ghostty](https://ghostty.org/), or [Kitty](https://sw.kovidgoyal.net/kitty/))
- Linux, macOS, or Windows

### Rust Toolchain

This project uses **Rust Edition 2024**, which requires **Rust 1.85 or later**. Install or update Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
rustc --version   # should be 1.85.0 or newer
```

## Dependencies

All crate dependencies are managed by [Cargo](https://doc.rust-lang.org/cargo/). Direct dependencies are declared in `Cargo.toml`; the full resolved dependency tree (including transitive crates) is pinned in `Cargo.lock`.

### Direct Dependencies

| Crate | Version | Purpose |
|---|---|---|
| [ratatui](https://crates.io/crates/ratatui) | `0.30.2` | Terminal UI framework (widgets, layout, styling, event loop) |
| [color-eyre](https://crates.io/crates/color-eyre) | `0.6` | Colorful error reporting and backtraces |
| [tui-big-text](https://crates.io/crates/tui-big-text) | `0.8` | Large pixel-style text rendering for the title banner |
| [arboard](https://crates.io/crates/arboard) | `3.6` | Cross-platform clipboard access for paste support |
| [reqwest](https://crates.io/crates/reqwest) | `0.12` | HTTP client for LLM API integration (planned) |
| [tokio](https://crates.io/crates/tokio) | `1` | Async runtime for API calls and streaming (planned) |
| [serde](https://crates.io/crates/serde) | `1` | Serialization/deserialization for API payloads (planned) |
| [serde_json](https://crates.io/crates/serde_json) | `1.0` | JSON encoding for API requests and responses (planned) |
| [futures-util](https://crates.io/crates/futures-util) | `0.3` | Async stream utilities for LLM response streaming (planned) |

#### `ratatui` features enabled

- `all-widgets` — includes all built-in Ratatui widgets (blocks, paragraphs, lists, tables, charts, etc.)

### Key Transitive Dependencies

These are pulled in automatically by the direct dependencies above:

| Crate | Role |
|---|---|
| [crossterm](https://crates.io/crates/crossterm) | Cross-platform terminal manipulation (input events, cursor, screen clearing) |
| [unicode-width](https://crates.io/crates/unicode-width) | Correct display width for Unicode characters |
| [compact_str](https://crates.io/crates/compact_str) | Efficient string storage for UI text |
| [cassowary](https://crates.io/crates/cassowary) | Constraint-based layout solver |
| [eyre](https://crates.io/crates/eyre) | Error handling (used by `color-eyre`) |
| [backtrace](https://crates.io/crates/backtrace) | Stack trace support for errors |

For the complete list of all resolved crates and their exact versions, see [`Cargo.lock`](./Cargo.lock).

## Getting Started

### Clone and Build

```bash
git clone <repository-url>
cd rusty_code
cargo build --release
```

### Run

```bash
cargo run
```

Or run the release binary directly:

```bash
./target/release/rusty_code
```

### Keyboard Shortcuts

| Key | Action |
|---|---|
| `e` | Enter editing mode |
| `Esc` | Return to normal mode |
| `Enter` | Submit input |
| `←` / `→` | Move cursor |
| `Backspace` | Delete character before cursor |
| `Ctrl+A` | Move cursor to start of line |
| `Ctrl+U` | Delete from cursor to start of line |
| `Ctrl+Z` | Undo last edit |
| `Ctrl+C` / `Cmd+C` | Clear input |
| `Ctrl+V` / `Cmd+V` | Paste from clipboard |
| `q` | Quit |

## Project Structure

```
src/
├── main.rs              # Entry point, event loop, and key handling
├── model.rs             # App state (input, cursor, mode, edit history)
├── view.rs              # UI rendering (title, help text, input area)
└── controller/
    ├── mod.rs           # Controller module
    ├── input.rs         # Character input, deletion, undo, paste, and clear
    └── cursor.rs        # Cursor movement logic
```

## Development

```bash
# Build in debug mode
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

CI runs `cargo build` and `cargo test` on every push and pull request to `main` (see [`.github/workflows/rust.yml`](./.github/workflows/rust.yml)).

## License

This project is licensed under the [Apache License 2.0](./LICENSE).
