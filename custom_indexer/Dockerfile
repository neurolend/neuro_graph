# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build the application
RUN cargo build --release --bin neurolend_indexer

# Runtime stage
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/neurolend_indexer ./neurolend_indexer

# Create output directory
RUN mkdir -p /app/output

# Run as non-root user
RUN useradd -r -s /bin/false indexer
RUN chown -R indexer:indexer /app
USER indexer

EXPOSE 8080

CMD ["./neurolend_indexer"]
