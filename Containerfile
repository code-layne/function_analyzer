FROM rust:alpine

LABEL org.opencontainers.image.title="function_analyzer"
LABEL org.opencontainers.image.source="https://github.com/code-layne/function_analyzer"
LABEL org.opencontainers.image.description="Rust polynomial/function analysis engine"
LABEL org.opencontainers.image.licenses="MIT"

WORKDIR /app
COPY --exclude=*/.git . /app/

RUN cargo build --release

CMD ["./target/release/function_analyzer"]
