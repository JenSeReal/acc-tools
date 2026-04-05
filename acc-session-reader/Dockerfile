# Build stage
FROM rust:1.94 AS builder

# Install trunk and wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

# Install Node.js for Tailwind CSS
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

WORKDIR /app

# Copy package files and install dependencies first (for better caching)
COPY package.json package-lock.json* ./
RUN npm install

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY acc-session-reader ./acc-session-reader
COPY acc-ui-components ./acc-ui-components

# Build Tailwind CSS
RUN npm run build:css

# Build the WASM frontend
WORKDIR /app/acc-session-reader
RUN trunk build --release

# Production stage - serve static files with nginx
FROM nginx:alpine

# Copy built assets
COPY --from=builder /app/acc-session-reader/dist /usr/share/nginx/html
COPY acc-session-reader/nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
