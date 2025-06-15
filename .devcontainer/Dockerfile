FROM rustlang/rust:nightly-bullseye

WORKDIR /workspaces/netpulse

RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev sqlite3 libsqlite3-dev curl gnupg rpm

# Install Speedtest CLI
RUN set -eux; \
    curl -Lo /tmp/speedtest.tgz https://install.speedtest.net/app/cli/ookla-speedtest-1.2.0-linux-x86_64.tgz; \
    tar -xzf /tmp/speedtest.tgz -C /usr/local/bin speedtest; \
    chmod +x /usr/local/bin/speedtest; \
    rm /tmp/speedtest.tgz

RUN rustup component add rustfmt

# Install cargo-deb and cargo-rpm for packaging
RUN cargo install cargo-deb cargo-rpm