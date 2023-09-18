# One Time Chat

A One Time Pad Messanger written in Rust and GTK4. Sucessor to the One Time Pad
UI project.

# Getting Started

```sh
# Build and develop for desktop
cargo tauri dev

# Build and release for desktop
cargo tauri build
```

## Prerequisites

```sh
# Tauri CLI
cargo install --locked tauri-cli

# Rust stable
rustup toolchain install stable

# WASM target
rustup target add wasm32-unknown-unknown

# Trunk WASM bundler
cargo install --locked trunk

# `wasm-bindgen` for Apple M1 chips (required by Trunk)
cargo install --locked wasm-bindgen-cli

# `esbuild` as dependency of `tauri-sys` crate (used in UI)
npm install --global --save-exact esbuild

# Optional: `tailwindcss` for UI styling
npm install --global tailwindcss
```

# License

    This file is part of One Time Chat.

    One Time Chat is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

    One Time Chat is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License along with One Time Chat. If not, see <https://www.gnu.org/licenses/>.
