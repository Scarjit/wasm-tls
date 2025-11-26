FROM rust:latest

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    clang \
    llvm \
    build-essential \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install wasm32 target and rustfmt
RUN rustup target add wasm32-unknown-unknown && \
    rustup component add rustfmt

# Set working directory
WORKDIR /app

# Copy project files
COPY . .

# Default command
CMD ["wasm-pack", "build", "--target", "web"]
