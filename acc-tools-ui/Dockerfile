# Build stage
FROM rust:1.95 AS builder

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
COPY Cargo.toml Cargo.lock ./
COPY acc-session-viewer/lib ./acc-session-viewer/lib
COPY acc-tools-ui-components ./acc-tools-ui-components
COPY motec-viewer/lib ./motec-viewer/lib
COPY acc-tools-ui ./acc-tools-ui

# Build the WASM frontend
WORKDIR /app/acc-tools-ui
RUN trunk build --release

# Production stage
FROM nginx:alpine

COPY --from=builder /app/acc-tools-ui/dist /usr/share/nginx/html
COPY acc-tools-ui/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
