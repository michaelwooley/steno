# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

# Development


Initial setup:

```bash
git clone git@github.com:michaelwooley/steno.git
cd steno

rustup target add wasm32-unknown-unknown
cargo install --locked trunk
cargo install wasm-bindgen-cli
```

To develop:

```bash
cargo tauri dev
```

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
