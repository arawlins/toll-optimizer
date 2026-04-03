# --- Stage 1: Build Frontend ---
FROM node:20-bookworm-slim AS frontend-builder
WORKDIR /app/frontend
COPY frontend/package*.json ./
RUN npm install
COPY frontend/ ./
RUN npm run build

# --- Stage 2: Build Backend ---
# Using the latest stable rust image to satisfy dependency requirements (e.g. time, home crates)
FROM rust:bookworm AS backend-builder
WORKDIR /app

# Create a dummy project to cache dependencies
RUN cargo new crates/core --lib --name common && \
    cargo new crates/api --bin --name toll-optimizer-api && \
    cargo new crates/cli --bin --name toll-optimizer-cli

COPY Cargo.toml Cargo.lock ./
COPY crates/core/Cargo.toml crates/core/
COPY crates/api/Cargo.toml crates/api/
COPY crates/cli/Cargo.toml crates/cli/
# Copy migrations for compile-time check
COPY crates/api/migrations/ crates/api/migrations/

# Build dependencies only
RUN cargo build --release -p toll-optimizer-api

# Copy actual source code
COPY crates/ crates/
# Touch the files to ensure cargo re-builds
RUN touch crates/core/src/lib.rs crates/api/src/main.rs

# Build the final binary
# Set SQLX_OFFLINE=true to bypass compile-time DB checks.
# KNOWN GAP: This project uses sqlx::query_as instead of the query! macros, so compile-time 
# verification is not utilized. SQLX_OFFLINE=true simply suppresses build errors.
ENV SQLX_OFFLINE=true
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
