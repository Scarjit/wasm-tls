.PHONY: all build clean serve

# Default target - show help
all: help

# Build the Wasm package
build:
	@echo "Building WebAssembly package..."
	wasm-pack build --target web

build-copy: build
	@echo "Copying WebAssembly package to www/pkg..."
	@cp -r pkg/* www/pkg/

# Build for release
release:
	@echo "Building WebAssembly package for release..."
	wasm-pack build --target web --release

# Serve
serve: build
	@echo "Starting HTTP server on http://localhost:3000"
	@mkdir -p www/pkg
	python3 -m http.server 3000 -d www

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf pkg target

# Install required dependencies
install-deps:
	@echo "Installing dependencies..."
	cargo install wasm-pack

# Format code
fmt:
	cargo fmt

# Run tests
test:
	wasm-pack test --node
