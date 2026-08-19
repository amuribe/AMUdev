# syntax=docker/dockerfile:1.7

FROM rust:1.95-bookworm AS builder

# cargo-leptos builds both the Axum server and the browser WASM bundle.
RUN rustup target add wasm32-unknown-unknown \
    && cargo install cargo-leptos --version 0.3.7 --locked

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY style ./style
COPY public ./public

# Cache compiled dependencies between builds, while copying the final artifacts
# outside the cache mount so the runtime stage can access them.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo leptos build --release \
    && cp target/release/amudev /app/amudev \
    && cp -r target/site /app/site

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder --chown=10001:10001 /app/amudev /app/amudev
COPY --from=builder --chown=10001:10001 /app/site /app/site

ENV LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_SITE_ROOT=/app/site

EXPOSE 3000

USER 10001:10001

CMD ["/app/amudev"]
