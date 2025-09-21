FROM rust:1.90.0-bullseye as builder

WORKDIR /usr/src/reth
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    libclang-dev \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create working directory
RUN mkdir -p /usr/src/reth/app
WORKDIR /usr/src/reth/app

# Copy the entire project
COPY ./reth/Cargo.toml ./Cargo.toml
COPY ./reth/src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/reth/app/target/release/reth-service /app/reth-service

# Create a directory for Reth data
RUN mkdir -p /app/data

# Expose the necessary ports
EXPOSE 8545 8546 30303 30303/udp

# Set the entrypoint
ENTRYPOINT ["/app/reth-service"]
