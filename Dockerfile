# ============================================================
# Stage 1: Build
# ============================================================
FROM rust:1.95-bookworm AS builder

WORKDIR /build

# Cache dependency builds
COPY Cargo.toml Cargo.lock* rust-toolchain.toml ./
COPY remem-core/Cargo.toml remem-core/Cargo.toml
COPY remem-mcp/Cargo.toml remem-mcp/Cargo.toml
COPY remem-api/Cargo.toml remem-api/Cargo.toml
COPY remem-cli/Cargo.toml remem-cli/Cargo.toml

# Create dummy src files for dependency caching
RUN mkdir -p remem-core/src remem-mcp/src remem-api/src remem-cli/src && \
    echo "pub fn main() {}" > remem-core/src/lib.rs && \
    echo "fn main() {}" > remem-mcp/src/main.rs && \
    echo "fn main() {}" > remem-api/src/main.rs && \
    echo "fn main() {}" > remem-cli/src/main.rs

RUN cargo build --release --workspace 2>/dev/null || true

# Copy real source and build
COPY remem-core/ remem-core/
COPY remem-mcp/ remem-mcp/
COPY remem-api/ remem-api/
COPY remem-cli/ remem-cli/

# Touch source files to invalidate cache
RUN touch remem-core/src/lib.rs remem-mcp/src/main.rs remem-api/src/main.rs remem-cli/src/main.rs

RUN cargo build --release --workspace

# ============================================================
# Stage 2: Runtime
# ============================================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1000 remem && \
    useradd --uid 1000 --gid remem --shell /bin/bash --create-home remem

COPY --from=builder /build/target/release/remem /usr/local/bin/remem
COPY --from=builder /build/target/release/remem-api /usr/local/bin/remem-api
COPY --from=builder /build/target/release/remem-mcp /usr/local/bin/remem-mcp

USER remem
WORKDIR /home/remem

# Default data directory
ENV REMEM_DATA_DIR=/home/remem/.remem
VOLUME ["/home/remem/.remem"]

# REST API port
EXPOSE 7474

# Default: run the REST API server
ENTRYPOINT ["remem"]
CMD ["serve", "--port", "7474"]

# ============================================================
# Labels
# ============================================================
LABEL org.opencontainers.image.source="https://github.com/rememhq/remem"
LABEL org.opencontainers.image.description="Reasoning memory layer for AI agents"
LABEL org.opencontainers.image.licenses="Apache-2.0"
LABEL org.opencontainers.image.vendor="rememhq"
