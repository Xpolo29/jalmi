# jalmi

A native desktop chat UI for local LLMs, built with [Iced](https://iced.rs/) and powered by [llama-swap](https://github.com/mostlygeek/llama-swap).

jalmi provides a clean, themed chat interface for interacting with locally-hosted language models through an OpenAI-compatible API. It supports streaming responses, multi-model management, and real-time model status tracking.

![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Features

- **Streaming responses** — Tokens are streamed in real-time via SSE (Server-Sent Events)
- **Multi-model support** — Switch between models via a dropdown, configured through `llama-swap`
- **Model lifecycle management** — Load, unload, and monitor model status from the UI
- **Chat history** — Full conversation context maintained across turns
- **Custom text editor** — Multi-line input with `Enter` to send and `Shift+Enter` for newlines, plus `Ctrl+Backspace`/`Ctrl+Delete` word deletion
- **Themed UI** — Consistent theming with rounded bubble styling, color-coded status indicators, and toggle buttons

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2021)
- [llama-swap](https://github.com/mostlygeek/llama-swap) running locally with models configured

## Configuration

jalmi reads its model list from the llama-swap configuration file located at:

```
~/.config/llama_swap.conf
```

The configuration file should be a YAML file with the following structure:

```yaml
health_check_timeout: 30
models:
  llama3-8b:
    cmd: "llama-server -m /path/to/llama3-8b-q4.gguf --port 8999"
    proxy: "http://127.0.0.1:8999"
    ttl: 300
  mistral-7b:
    cmd: "llama-server -m /path/to/mistral-7b-q4.gguf --port 9000"
    proxy: "http://127.0.0.1:9000"
    ttl: 300
profiles: {}
```

The available model names become the options in the jalmi dropdown selector.

## Building & Running

```bash
# Build
cargo build

# Run
cargo run
```

jalmi auto-detects the llama-swap port by looking up the `llama-swap` process on the host.

## Usage

1. **Select a model** from the dropdown at the top of the window
2. **Load the model** by clicking the **Load** button — the status indicator will turn orange ("Starting") then green ("Ready")
3. **Type a message** in the text box and press `Enter` to send
4. **Stream** — the response will appear in real-time as a left-aligned bubble
5. **Stop** an in-progress generation with the **Stop** button
6. **Retry** your last message with the **Retry** button
7. **Unload** a model when you're done to free resources

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| `Enter` | Send message |
| `Shift + Enter` | New line |
| `Ctrl + Backspace` | Delete word backwards |
| `Ctrl + Delete` | Delete word forwards |

## Architecture

```
src/
├── main.rs                     # App entry point, Iced Application trait, state & message handling
└── modules/
    ├── config/
    │   └── config.rs           # Reads llama-swap YAML config, discovers port
    ├── llm/
    │   └── llm.rs              # OpenAI-compatible API client with streaming support
    └── ui/
        ├── theme.rs            # Color palette, fonts, spacing constants
        └── widgets/
            ├── bubble.rs       # Chat bubble component (user/assistant alignment)
            ├── text_box.rs     # Multi-line text editor with custom key bindings
            ├── toggle_button.rs# Dual-state action button (Send/Stop, Load/Unload)
            └── status.rs       # Color-coded model status indicator
```

## Dependencies

| Crate | Purpose |
|---|---|
| [`iced`](https://crates.io/crates/iced) | Native GUI framework |
| [`iced_aw`](https://crates.io/crates/iced_aw) | Additional Iced widgets |
| [`ureq`](https://crates.io/crates/ureq) | HTTP client for API requests |
| [`serde_yaml`](https://crates.io/crates/serde_yaml) | YAML config parsing |
| [`listeners`](https://crates.io/crates/listeners) | Port discovery for llama-swap |
| [`colog`](https://crates.io/crates/colog) | Colored log output |

## License

MIT
