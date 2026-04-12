# GitRadar Justfile - Task runner for development

# Default recipe
default:
    @just --list

# Setup development environment
setup:
    #!/usr/bin/env bash
    set -e
    echo "Setting up GitRadar development environment..."
    
    # Check if Bun is installed
    if ! command -v bun &> /dev/null; then
        echo "Installing Bun..."
        curl -fsSL https://bun.sh/install | bash
    fi
    
    # Check if Rust is installed
    if ! command -v cargo &> /dev/null; then
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    fi
    
    # Install dependencies
    echo "Installing dependencies..."
    bun install
    
    echo "Setup complete!"

# Start development server
dev:
    @echo "Starting GitRadar in development mode..."
    bun run tauri dev

# Build for production
build:
    @echo "Building GitRadar for production..."
    bun run tauri build

# Run tests
test:
    #!/usr/bin/env bash
    set -e
    echo "Running tests..."
    
    # TypeScript check
    echo "TypeScript type check..."
    bun run check
    
    # Rust tests
    echo "Rust tests..."
    cd src-tauri && cargo test
    
    echo "All tests passed!"

# Clean build artifacts
clean:
    #!/usr/bin/env bash
    set -e
    echo "Cleaning build artifacts..."
    
    # Frontend
    rm -rf dist/
    rm -rf node_modules/.vite
    
    # Rust
    cd src-tauri && cargo clean
    
    echo "Cleanup complete!"

# Package application
package:
    @echo "Packaging GitRadar..."
    just build
    cd src-tauri && cargo tauri build

# Check code formatting
fmt:
    #!/usr/bin/env bash
    set -e
    echo "Formatting code..."
    
    # Rust formatting
    cd src-tauri && cargo fmt
    
    echo "Code formatted!"

# Run linter
lint:
    #!/usr/bin/env bash
    set -e
    echo "Running linters..."
    
    # Rust clippy
    cd src-tauri && cargo clippy -- -D warnings
    
    echo "Linting complete!"

# Show project info
info:
    @echo "GitRadar - Git Analytics Desktop App"
    @echo ""
    @echo "Available commands:"
    @echo "  just setup    - Setup development environment"
    @echo "  just dev      - Start development server"
    @echo "  just build    - Build for production"
    @echo "  just test     - Run tests"
    @echo "  just clean    - Clean build artifacts"
    @echo "  just package  - Package application"
    @echo "  just fmt      - Format code"
    @echo "  just lint     - Run linter"
    @echo "  just info     - Show this help"

# Watch for changes and rebuild
watch:
    @echo "Watching for file changes..."
    @echo "Use Ctrl+C to stop"
    @while true; do
        find src/ src-tauri/src/ -name "*.rs" -o -name "*.ts" -o -name "*.tsx" | \
        entr -d -r just dev
    done
