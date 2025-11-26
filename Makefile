.PHONY: all build release clean serve test fmt build-copy help

# Default target - show help
all: help

# Build the Wasm package using Docker/Podman
build:
	@echo "Building WebAssembly package with Docker..."
	@mkdir -p pkg
	podman build -t wasm-tls-builder .
	podman run --rm -v $(PWD)/pkg:/app/pkg wasm-tls-builder

# Build and copy to www/pkg
build-copy: build
	@echo "Copying WebAssembly package to www/pkg..."
	@mkdir -p www/pkg
	@cp -r pkg/* www/pkg/

# Build for release using Docker/Podman
release:
	@echo "Building WebAssembly package for release with Docker..."
	@mkdir -p pkg
	podman build -t wasm-tls-builder .
	podman run --rm -v $(PWD)/pkg:/app/pkg wasm-tls-builder sh -c "wasm-pack build --target web --release"

# Serve
serve: build
	@echo "Starting HTTP server on http://localhost:3000"
	@mkdir -p www/pkg
	@cp -r pkg/* www/pkg/
	python3 -m http.server 3000 -d www

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	rm -rf pkg target

# Format code
fmt:
	@echo "Formatting code with Docker..."
	podman build -t wasm-tls-builder .
	podman run --rm -v $(PWD):/app wasm-tls-builder cargo fmt

# Run tests
test:
	@echo "Running tests with Docker..."
	podman build -t wasm-tls-builder .
	podman run --rm -v $(PWD):/app wasm-tls-builder wasm-pack test --node

# Help
help:
	@echo "Available targets:"
	@echo "  build         - Build using Docker/Podman"
	@echo "  release       - Release build using Docker/Podman"
	@echo "  build-copy    - Build and copy to www/pkg"
	@echo "  serve         - Build and start HTTP server on port 3000"
	@echo "  test          - Run tests using Docker/Podman"
	@echo "  fmt           - Format code using Docker/Podman"
	@echo "  clean         - Remove build artifacts"
	@echo "  help          - Show this help message"
