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
    && rm -rf /var/lib/apt/lists/*

# Add MUSL target
RUN rustup target add x86_64-unknown-linux-musl

# Build backend for MUSL
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

