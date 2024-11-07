FROM rust:1.75-slim as builder

WORKDIR /app

RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/lemon-apod-api /usr/local/bin/

EXPOSE 50051

CMD ["lemon-apod-api"]