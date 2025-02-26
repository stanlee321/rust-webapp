.PHONY: all run build clean check-trunk install-trunk check-wasm install-wasm frontend backend build-binary package run-binary

# Default target
all: run

# Main run command
run: check-trunk check-wasm
	@echo "Starting the application (backend and frontend)..."
	@mkdir -p scripts
	@[ -f scripts/run.sh ] || echo '#!/bin/bash\ntrap "kill 0" EXIT\n\n# Start the backend first\ncargo run &\nbackend_pid=$$!\n\n# Give the backend time to start up\necho "Waiting for backend to start..."\nsleep 3\n\n# Then start the frontend with proxy configuration\ncd frontend && trunk serve --proxy-backend=http://127.0.0.1:3000/api --address 127.0.0.1 --port 9090 &\nfrontend_pid=$$!\n\n# Keep the script running until both processes exit\nwait $$backend_pid $$frontend_pid' > scripts/run.sh
	@chmod +x scripts/run.sh
	@./scripts/run.sh

# Check if trunk is installed
check-trunk:
	@which trunk > /dev/null || (echo "Trunk is not installed. Running 'make install-trunk' to install it." && make install-trunk)

# Install trunk and wasm-bindgen-cli
install-trunk:
	@echo "Installing Trunk and wasm-bindgen-cli..."
	@cargo install trunk wasm-bindgen-cli
	@echo "Trunk and wasm-bindgen-cli installed successfully!"

# Check if wasm32 target is installed
check-wasm:
	@rustup target list | grep "wasm32-unknown-unknown (installed)" > /dev/null || (echo "WebAssembly target is not installed. Running 'make install-wasm' to install it." && make install-wasm)

# Install wasm32 target
install-wasm:
	@echo "Installing wasm32-unknown-unknown target..."
	@rustup target add wasm32-unknown-unknown
	@echo "WebAssembly target installed successfully!"

# Build the frontend
frontend:
	@echo "Building Yew frontend with Trunk..."
	@mkdir -p frontend/dist
	@cd frontend && trunk build --release
	@echo "Frontend built successfully!"

# Run the backend server
backend:
	@echo "Starting the Axum backend server..."
	@cargo run

# Just build everything without running
build: frontend
	@cargo build --release
	@echo "Build completed successfully!"

# Clean up build artifacts
clean:
	@echo "Cleaning up build artifacts..."
	@rm -rf frontend/dist
	@rm -rf target
	@echo "Clean completed successfully!"

# Development mode (watch for changes)
dev:
	@echo "Starting development server with hot reloading..."
	@mkdir -p scripts
	@[ -f scripts/dev.sh ] || echo '#!/bin/bash\ntrap "kill 0" EXIT\ncd frontend && trunk serve --proxy-backend=http://127.0.0.1:3000/api --address 127.0.0.1 --port 9090 &\ncargo watch -x run &\nwait' > scripts/dev.sh
	@chmod +x scripts/dev.sh
	@./scripts/dev.sh

# Show help
help:
	@echo "Available commands:"
	@echo "  make run         - Start both the backend and frontend servers simultaneously (default)"
	@echo "  make build       - Build the project without running"
	@echo "  make frontend    - Build only the Yew frontend"
	@echo "  make backend     - Run only the backend server"
	@echo "  make dev         - Start development servers with hot reloading"
	@echo "  make clean       - Remove build artifacts"
	@echo "  make help        - Show this help message"

# Build a production-ready binary
build-binary: check-wasm
	@echo "Building production binary..."
	@echo "Step 1: Building frontend..."
	@mkdir -p frontend/dist
	@cd frontend && trunk build --release
	@echo "Step 2: Building optimized backend binary..."
	@cargo build --release
	@echo "Creating deployment package..."
	@mkdir -p dist
	@cp target/release/rust-web-app dist/
	@cp -r frontend/dist dist/frontend-assets
	@echo "Binary built successfully! The executable is in 'dist/rust-web-app'"
	@echo "To run the binary: ./dist/rust-web-app"
	@echo ""
	@echo "Environment variables for configuration:"
	@echo "  PORT=<number>              - Specify port to listen on (default: tries 3000, 3001, etc.)"
	@echo "  HOST=0.0.0.0               - Listen on all interfaces (default: 127.0.0.1)"
	@echo "  FRONTEND_PATH=./assets     - Custom path to frontend assets (default: frontend/dist)"
	@echo "  LOG_LEVEL=debug            - Set log level (default: info)"
	@echo ""
	@echo "Example: PORT=8080 HOST=0.0.0.0 ./dist/rust-web-app"

# Create a deployable package (tar.gz)
package: build-binary
	@echo "Creating deployable package..."
	@tar -czf rust-web-app.tar.gz -C dist .
	@echo "Package created: rust-web-app.tar.gz"
	@echo "To deploy, extract the archive and run the executable"
	@echo ""
	@echo "Example deployment commands:"
	@echo "  mkdir -p /opt/rust-web-app"
	@echo "  tar -xzf rust-web-app.tar.gz -C /opt/rust-web-app"
	@echo "  cd /opt/rust-web-app"
	@echo "  PORT=8080 HOST=0.0.0.0 FRONTEND_PATH=./frontend-assets ./rust-web-app"

# Run the production binary locally for testing
run-binary: build-binary
	@echo "Running the production binary..."
	@cd dist && ./rust-web-app 