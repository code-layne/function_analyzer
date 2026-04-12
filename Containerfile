# syntax=docker/dockerfile:1

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# 1. Generate dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2. Build dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 3. Build application
COPY . .
RUN cargo build --release --bin function_analyzer

# 4. Minimal runtime image
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="function_analyzer" \
      org.opencontainers.image.source="https://github.com/code-layne/function_analyzer" \
      org.opencontainers.image.description="Rust polynomial/function analysis engine" \
      org.opencontainers.image.licenses="MIT"

WORKDIR /app

# Optional but useful for TLS/cert validation if your app ever makes HTTPS requests
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/function_analyzer /usr/local/bin/function_analyzer

CMD ["function_analyzer"]