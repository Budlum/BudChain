# ── Budlum Core Production Docker Image ────────────────────
# Multi-stage build: builder → runtime

# ── Stage 1: Builder ────────────────────────────────────────
FROM rust:1.94.0-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    protobuf-compiler \
    clang \
    cmake \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Clone BudZKVM dependency (sibling dir required by Cargo.toml path dep)
RUN git clone https://github.com/Budlum/BudZKVM.git /BudZKVM

# Copy source and manifests
COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/
COPY benches/ ./benches/
COPY proto/ ./proto/

# Build release binary
RUN cargo build --release --locked && \
    cp target/release/budlum-core /usr/local/bin/budlum-core

# ── Stage 2: Runtime ────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/bin/budlum-core /usr/local/bin/budlum-core

RUN useradd --create-home --shell /bin/bash budlum
USER budlum
WORKDIR /home/budlum

# Expose default ports
#   4001 = P2P (devnet), 8545 = RPC public, 8546 = RPC operator, 9090 = Metrics
EXPOSE 4001 8545 8546 9090

ENV RUST_LOG=info

ENTRYPOINT ["budlum-core"]
CMD ["--network", "devnet", "--port", "4001"]
