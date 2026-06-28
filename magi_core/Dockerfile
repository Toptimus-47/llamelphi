# Stage 1: Build
FROM rust:1.74-slim-bookworm AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev cmake g++
COPY . .

# Build the release binary
RUN cargo build --release --bin elite_server

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binary and required assets
COPY --from:builder /app/target/release/elite_server /app/
COPY --from:builder /app/vector_db /app/vector_db
COPY --from:builder /app/models /app/models

EXPOSE 8000
CMD ["./elite_server"]
