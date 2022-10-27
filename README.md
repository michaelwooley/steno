# ⌨️ steno

[![Test](https://github.com/michaelwooley/steno/actions/workflows/test.yml/badge.svg)](https://github.com/michaelwooley/steno/actions/workflows/test.yml) [![Release](https://github.com/michaelwooley/steno/actions/workflows/release.yml/badge.svg?event=release)](https://github.com/michaelwooley/steno/actions/workflows/release.yml)

![Preview](./public/operation-preview.gif)

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



## CI Notes

Based on:

- https://github.com/probablykasper/tauri-svelte-template
- https://github.com/Uninen/tauri-vue-template/tree/main/.github

https://tauri.app/v1/guides/faq#node-or-cargo

https://github.com/tauri-apps/tauri-action

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).
