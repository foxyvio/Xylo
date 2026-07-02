# Build stage
FROM rust:1.75-slim-bookworm as builder

WORKDIR /app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/xylo-core /app/xylo-core
COPY .env.example .env

EXPOSE 3000

CMD ["./xylo-core"]
