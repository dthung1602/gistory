# Multi-stage build for a Rust webserver targeting Cloud Run

# 1) Build stage
FROM rust:1.90.0-slim-trixie AS builder

WORKDIR /app

# System deps commonly needed by Rust crates (adjust if your deps differ)
RUN apt-get update && apt-get install -y --no-install-recommends build-essential curl pkg-config libssl-dev libsqlite3-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Pre-cache dependencies
# Copy workspace manifests first (adjust if your workspace layout differs)
COPY Cargo.toml Cargo.lock ./
COPY webserver/Cargo.toml webserver/Cargo.toml

# Create a dummy source to allow dependency resolution caching
RUN mkdir -p webserver/src && \
    printf 'fn main(){println!("dummy build");}\n' > webserver/src/main.rs

# Try to build the webserver to cache deps (ignore failure if features differ)
RUN cargo build -p webserver --release || true

# 2) Copy full source and build
COPY . .
RUN cargo build -p webserver --release && strip target/release/webserver

# Install diesel
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

# Run migration to create empty sqlite db
WORKDIR webserver
ENV DATABASE_URL=./database.db
RUN diesel migration run

# 3) Runtime stage
FROM debian:trixie-slim AS runtime

WORKDIR /app
ENV RUST_LOG=INFO
ENV PORT=3000

# Copy the built binary (assumes crate/package name is "webserver")
COPY --from=builder /app/target/release/webserver /usr/local/bin/webserver

# Copy empty sqlite3 db
RUN mkdir /data
COPY --from=builder /app/webserver/database.db /data/database.db
ENV DATABASE_URL=/data/database.db

EXPOSE 3000

# Ensure your server binds to 0.0.0.0:$PORT inside the container.
CMD ["webserver"]
