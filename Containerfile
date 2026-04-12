# syntax=docker/dockerfile:1

FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# 1) Generate the dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2) Build dependencies only
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# 3) Build the actual application
COPY . .
RUN cargo build --release --bin function_analyzer

# 4) Minimal runtime image
FROM alpine:3.20 AS runtime
LABEL org.opencontainers.image.title="function_analyzer"
LABEL org.opencontainers.image.source="https://github.com/code-layne/function_analyzer"
LABEL org.opencontainers.image.description="Rust polynomial/function analysis engine"
LABEL org.opencontainers.image.licenses="MIT"
WORKDIR /app
RUN apk add --no-cache libgcc

COPY --from=builder /app/target/release/function_analyzer /usr/local/bin/function_analyzer

CMD ["function_analyzer"]