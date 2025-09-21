FROM rust:1.90.0-bullseye as builder

WORKDIR /usr/src/ui
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libx11-dev \
    libxcb1-dev \
    libxcb-render0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new Cargo project
RUN cargo new --lib ui-service
WORKDIR /usr/src/ui/ui-service

# Copy over the Cargo manifest
COPY ./ui/Cargo.toml ./Cargo.toml

# Create dummy source files to satisfy the build
RUN mkdir -p src && \
    echo "// Dummy lib file for dependency build" > src/lib.rs && \
    echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs

# Build dependencies - this is the caching Docker layer
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./ui/src ./src
COPY ./ui/assets ./assets

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libx11-6 \
    libxcb1 \
    libxcb-render0 \
    libxcb-shape0 \
    libxcb-xfixes0 \
    libpq5 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary and assets from the builder stage
COPY --from=builder /usr/src/ui/ui-service/target/release/ui-service /app/ui-service
COPY --from=builder /usr/src/ui/ui-service/assets /app/assets

# Expose the necessary ports
EXPOSE 8000

# Set the entrypoint
ENTRYPOINT ["/app/ui-service"]
