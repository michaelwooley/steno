# Svelte-Kit + Vite

This template should help get you started developing with Tauri and Svelte-Kit in Vite.

# Development

In addition to [prereqs](https://tauri.app/v1/guides/getting-started/prerequisites), go ahead and:


```bash
cargo install tauri-cli # Use for dev
```

To run locally:

```bash
git clone ...

cd steno

npm i
cargo tauri dev
```


## Testing


```bash
npm test
cargo test --manifest-path=src-tauri/Cargo.toml
```


## CI Notes

Based on:

- https://github.com/probablykasper/tauri-svelte-template
- https://github.com/Uninen/tauri-vue-template/tree/main/.github

https://tauri.app/v1/guides/faq#node-or-cargo

https://github.com/tauri-apps/tauri-action

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

