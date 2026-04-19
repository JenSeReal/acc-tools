# ── Stage 1: Build WASM frontend ─────────────────────────────────────────────
FROM docker.io/library/rust:1.95 AS wasm-builder

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Install Node.js for Tailwind CSS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

WORKDIR /app

# npm dependencies first (better layer caching)
COPY package.json package-lock.json* ./
RUN npm install

# Workspace files
COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY acc-session-viewer/lib ./acc-session-viewer/lib
COPY acc-tools-ui-components ./acc-tools-ui-components
COPY motec-viewer/lib ./motec-viewer/lib
COPY acc-tools-ui ./acc-tools-ui

# Build the WASM frontend (csr + web features for HTTP bridge)
WORKDIR /app/acc-tools-ui
RUN trunk build --release

# ── Stage 2: Build Axum server ────────────────────────────────────────────────
FROM docker.io/library/rust:1.95 AS server-builder

WORKDIR /app

COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY acc-session-viewer/lib ./acc-session-viewer/lib
COPY acc-tools-ui-components ./acc-tools-ui-components
COPY motec-viewer/lib ./motec-viewer/lib
COPY acc-tools-ui ./acc-tools-ui

RUN cargo build --release -p acc-tools-ui-server

# ── Stage 3: Runtime ──────────────────────────────────────────────────────────
FROM docker.io/library/debian:bookworm-slim

RUN apt-get update && apt-get install -y libfontconfig1 && rm -rf /var/lib/apt/lists/*

COPY --from=wasm-builder /app/acc-tools-ui/dist /app/dist
COPY --from=server-builder /app/target/release/acc-tools-ui-server /usr/local/bin/

ENV STATIC_DIR=/app/dist
ENV DATABASE_PATH=/data/sessions.db
ENV PORT=8080

VOLUME ["/data"]
EXPOSE 8080
CMD ["acc-tools-ui-server"]
