# ===== Builder stage =====
FROM rust:latest AS builder

# Set working directory
WORKDIR /app

# Copy backend and frontend source files
COPY . .

# Install dependencies needed for building
RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    musl-tools \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js and npm (needed for Trunk)
RUN curl -fsSL https://deb.nodesource.com/setup_18.x | bash - \
    && apt-get install -y nodejs

# Add MUSL target and wasm32 target for frontend
RUN rustup target add x86_64-unknown-linux-musl wasm32-unknown-unknown

# Install Trunk for building the frontend
RUN cargo install trunk

# Build frontend first
WORKDIR /app/frontend
RUN trunk build --release

# Build backend for MUSL
WORKDIR /app
RUN cargo build --release --target x86_64-unknown-linux-musl -p backend

# ===== Runtime stage =====
FROM debian:bookworm-slim

# Set working directory
WORKDIR /app

# Copy the statically-linked MUSL binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/backend ./backend

# Copy frontend assets
COPY --from=builder /app/frontend/dist ./frontend/dist

# Run the backend binary
CMD ["./backend"]

