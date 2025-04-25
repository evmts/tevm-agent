# Builder stage for Rust
FROM rust:slim as rust-builder
WORKDIR /app

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    git \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust project files
COPY Cargo.toml Cargo.lock* ./
COPY src ./src

# Build the Rust application in release mode
RUN cargo build --release

# Node.js build stage for Tevm
FROM node:20 as node-builder
WORKDIR /tevm

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install pnpm and bun
RUN npm install -g pnpm
RUN curl -fsSL https://bun.sh/install | bash
ENV PATH="/root/.bun/bin:${PATH}"

# Clone Tevm repository
RUN git clone https://github.com/evmts/tevm-monorepo.git .

# Install dependencies and build packages
RUN pnpm install
RUN pnpm nx run-many --targets=build:rust,build:app,build:dist,build:types

# Final stage
FROM ubuntu:24.04
WORKDIR /app

# Install Node.js
RUN apt-get update && apt-get install -y \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install -y nodejs

# Install Claude Code CLI globally
RUN npm install -g @anthropic-ai/claude-code

# Copy our Rust CLI binary
COPY --from=rust-builder /app/target/release/tevm-cli /usr/local/bin/tevm-cli
RUN chmod +x /usr/local/bin/tevm-cli

# Copy Tevm files
COPY --from=node-builder /tevm /tevm

# Set the working directory to Tevm repo
WORKDIR /tevm

# Make tevm-cli available on PATH
ENV PATH="/usr/local/bin:${PATH}"

# Create an entrypoint script
RUN echo '#!/bin/bash\nexec tevm-cli "$@"' > /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]