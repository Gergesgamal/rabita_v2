# syntax=docker/dockerfile:1
FROM rust:bookworm AS builder
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release -p backend --bin backend \
    && cp target/release/backend /usr/local/bin/backend

FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/backend /usr/local/bin/backend
EXPOSE 3000
CMD ["backend"]
