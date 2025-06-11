FROM rust:1.77-bullseye as builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev curl gnupg

# Install Speedtest CLI (Ookla)
RUN curl -s https://install.speedtest.net/app/cli/install.deb.sh | bash && \
    apt-get install -y speedtest

# Create app directory
WORKDIR /app

# Copy source and build
COPY . .
RUN cargo build --release

# ---- Runtime image ----
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y libssl3 ca-certificates sqlite3 && \
    rm -rf /var/lib/apt/lists/*

# Install Speedtest CLI (Ookla) in runtime image
RUN apt-get update && \
    apt-get install -y curl gnupg && \
    curl -s https://install.speedtest.net/app/cli/install.deb.sh | bash && \
    apt-get install -y speedtest && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary and static files
COPY --from=builder /app/target/release/netpulse /app/netpulse
COPY --from=builder /app/static /app/static
COPY --from=builder /app/templates /app/templates

# Expose port
EXPOSE 3000

# Run the application
CMD ["./netpulse"]