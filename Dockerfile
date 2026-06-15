# Stage 1: Build the application
FROM rust:1.90-slim AS builder

WORKDIR /usr/src/app

# Install build dependencies for OpenSSL/bcrypt
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build the release binary
RUN cargo build --release

# Stage 2: Create a minimal runner image
FROM ubuntu:22.04 AS runner

WORKDIR /usr/local/bin

# Avoid timezone/interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install runtime dependencies (OpenSSL & Certificates for HTTPS/TMDb API calls)
RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/movie-website /usr/local/bin/movie-website

# Copy the public directory (static frontend assets)
COPY public ./public

# Expose port (Render overrides this with PORT env var, default to 8080)
EXPOSE 8080

# Run the app
CMD ["movie-website"]
