FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/app*

COPY src ./src

RUN cargo build --locked --release && \
    cp target/release/app /app/app

FROM gcr.io/distroless/cc-debian12:nonroot

COPY --chown=10001:10001 --from=builder /app/app /app

USER 10001:10001

EXPOSE 3000

HEALTHCHECK --interval=10s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/app"]

ENTRYPOINT ["/app"]
