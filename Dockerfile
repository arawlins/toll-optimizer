# --- Stage 1: Build Frontend ---
FROM node:20-bookworm-slim AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# --- Stage 2: Build Backend ---
FROM rust:1.85-bookworm AS backend-builder
WORKDIR /app

# Create a dummy project to cache dependencies
RUN cargo new --lib crates/core && \
    cargo new --bin toll-optimizer-api --path crates/api && \
    cargo new --bin toll-optimizer-cli --path crates/cli

COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml crates/core/
COPY crates/api/Cargo.toml crates/api/
COPY crates/cli/Cargo.toml crates/cli/

# Build dependencies only
RUN cargo build --release -p toll-optimizer-api

# Copy actual source code
COPY crates/ crates/
# Touch the files to ensure cargo re-builds
RUN touch crates/core/src/lib.rs crates/api/src/main.rs

# Build the final binary
RUN cargo build --release -p toll-optimizer-api

# --- Stage 3: Runtime ---
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies (SSL certificates for potential HTTPS requests)
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy binary from backend builder
COPY --from=backend-builder /app/target/release/toll-optimizer-api /app/toll-optimizer-api

# Copy frontend assets from frontend builder
COPY --from=frontend-builder /app/frontend/dist /app/dist

# Expose the API port
EXPOSE 3000

# Set environment variables
ENV RUST_LOG=info

# Run the binary
CMD ["./toll-optimizer-api"]
