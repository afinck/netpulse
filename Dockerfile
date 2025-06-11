FROM rustlang/rust:nightly-bullseye as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev curl gnupg

# Download and install Speedtest CLI (Ookla) directly
RUN set -eux; \
    curl -Lo /tmp/speedtest.tgz https://install.speedtest.net/app/cli/ookla-speedtest-1.2.0-linux-x86_64.tgz; \
    tar -xzf /tmp/speedtest.tgz -C /usr/local/bin speedtest; \
    chmod +x /usr/local/bin/speedtest; \
    rm /tmp/speedtest.tgz

# Create app directory
WORKDIR /app

# Copy source and build
COPY . .
RUN cargo build --release

# ---- Runtime image ----
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates sqlite3 curl && \
    rm -rf /var/lib/apt/lists/*

# Download and install Speedtest CLI (Ookla) directly
RUN set -eux; \
    curl -Lo /tmp/speedtest.tgz https://install.speedtest.net/app/cli/ookla-speedtest-1.2.0-linux-x86_64.tgz; \
    tar -xzf /tmp/speedtest.tgz -C /usr/local/bin speedtest; \
    chmod +x /usr/local/bin/speedtest; \
    rm /tmp/speedtest.tgz

WORKDIR /app

# Copy the built binary and static files
COPY --from=builder /app/target/release/netpulse /app/netpulse
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates

# Expose port
EXPOSE 3000

# Run the application
CMD ["./netpulse"]