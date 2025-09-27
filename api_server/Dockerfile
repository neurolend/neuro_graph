# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY Cargo.toml ./
COPY src ./src

# Build the application
RUN cargo build --release --bin api_server

# Runtime stage
FROM debian:bookworm-slim

# Install required dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/api_server ./api_server

# Run as non-root user
RUN useradd -r -s /bin/false apiuser
RUN chown -R apiuser:apiuser /app
USER apiuser

EXPOSE 3001

CMD ["./api_server"]
