# syntax=docker/dockerfile:1

# ============================================================================
# Stage 1: Chef - Prepare dependency cache
# ============================================================================
FROM rust:1.94.0-bookworm AS chef

RUN cargo install cargo-chef --locked
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# ============================================================================
# Stage 2: Planner - Generate recipe.json for dependency caching
# ============================================================================
FROM chef AS planner

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY acc-race-session-reader-lib ./acc-race-session-reader-lib
COPY acc-race-session-reader-frontend/Cargo.toml ./acc-race-session-reader-frontend/
COPY acc-race-session-reader-frontend/src ./acc-race-session-reader-frontend/src

RUN cargo chef prepare --recipe-path recipe.json

# ============================================================================
# Stage 3: Builder - Build dependencies and application
# ============================================================================
FROM chef AS builder

# Install trunk
RUN cargo install trunk --locked

# Copy recipe and build dependencies first (cached layer)
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --target wasm32-unknown-unknown

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY acc-race-session-reader-lib ./acc-race-session-reader-lib
COPY acc-race-session-reader-frontend ./acc-race-session-reader-frontend

# Build the frontend with trunk
WORKDIR /app/acc-race-session-reader-frontend
RUN trunk build --release

# ============================================================================
# Stage 4: Runtime - Serve static files with nginx
# ============================================================================
FROM nginx:alpine AS runtime

# Copy the built static files
COPY --from=builder /app/acc-race-session-reader-frontend/dist /usr/share/nginx/html

# Copy custom nginx config for SPA routing
COPY --from=builder /app/acc-race-session-reader-frontend/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
