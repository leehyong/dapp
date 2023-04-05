# Dapp

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Requirements
### [rust][rust]
### [dart_sass][dart_sass]
### [trunk][trunk]
### [tauri][tauri]

## Development
  ```
  rustup target add wasm32-unknown-unknown
  cargo install trunk
  ```
## web 
### develop: `trunk serve` 
### build: `trunk build`

## desktop
### develop: `cargo tauri dev`
### build: `cargo tauri build`


[Tauri]: https://tauri.app/v1/guides/getting-started/prerequisites
[yew]: https://yew.rs/
[trunk]: https://trunkrs.dev/
[dart_sass]: https://github.com/sass/dart-sass
[rust]: https://www.rust-lang.org/