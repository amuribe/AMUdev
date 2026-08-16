# AMUdev

[![CI](https://github.com/OWNER/REPOSITORY/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/amuribe/AMUdev/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-1.95-informational)

<picture>
<source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
<img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos Axum Starter Template

This is a template for use with the [Leptos](https://github.com/leptos-rs/leptos) web framework and the [cargo-leptos](https://github.com/akesson/cargo-leptos) tool using [Axum](https://github.com/tokio-rs/axum).

## Creating your template repo

If you don't have `cargo-leptos` installed you can install it with

```bash
cargo install cargo-leptos --locked
```

Then run

```bash
cargo leptos new --git https://github.com/leptos-rs/start-axum
```

to generate a new project template.

```bash
cd amudev
```

to go to your newly created project.
Feel free to explore the project structure, but the best place to start with your application code is in `src/app.rs`.
Additionally, Cargo.toml may need updating as new versions of the dependencies are released, especially if things are not working after a `cargo update`.

## Running your project

```bash
cargo leptos watch
```

## Installing Additional Tools

This project builds with stable Rust. Install the WebAssembly target and the same `cargo-leptos` version used by the Dockerfile:

```bash
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos --version 0.3.7 --locked
```

`cargo-leptos` downloads its Sass and WebAssembly optimization helpers when needed. End-to-end tests additionally require `npm ci` in the `end2end` directory and the Playwright browser dependencies.

## Compiling for Release

```bash
cargo leptos build --release
```

This generates the server binary at `target/release/amudev` and the browser assets in `target/site`.

## Running with Docker

Build the production image from the repository root:

```bash
docker build --tag amudev:local .
```

Run it and publish the application on port 3000:

```bash
docker run --rm --publish 3000:3000 amudev:local
```

Then open <http://localhost:3000>. The image uses a build stage for Rust, `cargo-leptos`, and WebAssembly, then copies only the server binary and generated site into the runtime stage. It runs as an unprivileged user.

The Dockerfile pins Rust 1.95 and `cargo-leptos` 0.3.7 so container builds do not silently change when new toolchain versions are released. Local development and hot reload still use `cargo leptos watch` directly; a containerized development workflow can be added separately if it becomes useful.

## Testing Your Project

```bash
cargo leptos end-to-end
```

```bash
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.
Tests are located in end2end/tests directory.

## Executing a Server on a Remote Machine Without the Toolchain

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located at `target/release/amudev`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
amudev
site/
```

Set the following environment variables (updating for your project as needed):

```sh
export LEPTOS_OUTPUT_NAME="amudev"
export LEPTOS_SITE_ROOT="site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.

## Licensing

This template itself is released under the Unlicense. You should replace the LICENSE for your own application with an appropriate license if you plan to release it publicly.
