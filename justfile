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

# Format all code (TypeScript + Rust)
fmt:
    #!/usr/bin/env bash
    set -e
    echo "Formatting code..."
    
    # TypeScript/React formatting
    echo "Formatting TypeScript/React..."
    bun run format
    
    # Rust formatting
    echo "Formatting Rust..."
    cd src-tauri && cargo fmt
    
    echo "All code formatted!"

# Check formatting without fixing
fmt-check:
    #!/usr/bin/env bash
    set -e
    echo "Checking code formatting..."
    
    # TypeScript/React format check
    echo "Checking TypeScript/React formatting..."
    bun run format:check
    
    # Rust format check
    echo "Checking Rust formatting..."
    cd src-tauri && cargo fmt -- --check
    
    echo "Formatting check complete!"

# Run all linters
lint:
    #!/usr/bin/env bash
    set -e
    echo "Running linters..."
    
    # TypeScript/React linting
    echo "Linting TypeScript/React..."
    bun run lint
    
    # Rust clippy
    echo "Linting Rust..."
    cd src-tauri && cargo clippy -- -D warnings
    
    echo "All linting complete!"

# Fix linting issues
lint-fix:
    #!/usr/bin/env bash
    set -e
    echo "Fixing linting issues..."
    
    # TypeScript/React lint fix
    echo "Fixing TypeScript/React issues..."
    bun run lint:fix
    
    # Rust clippy fixes (limited)
    echo "Checking Rust for auto-fixable issues..."
    cd src-tauri && cargo clippy --fix --allow-dirty -- -D warnings
    
    echo "Lint fixes applied!"

# Comprehensive type checking
type-check:
    #!/usr/bin/env bash
    set -e
    echo "Running type checks..."
    
    # TypeScript type check
    echo "TypeScript type check..."
    bun run type-check
    
    # Rust type check
    echo "Rust type check..."
    cd src-tauri && cargo check
    
    echo "Type checks complete!"

# Complete code quality check
check:
    #!/usr/bin/env bash
    set -e
    echo "Running complete code quality check..."
    
    # Run all checks
    just type-check
    just fmt-check
    just lint
    
    echo "All checks passed!"

# Show project info
info:
    @echo "GitRadar - Git Analytics Desktop App"
    @echo ""
    @echo "Available commands:"
    @echo "  just setup      - Setup development environment"
    @echo "  just dev        - Start development server"
    @echo "  just build      - Build for production"
    @echo "  just test       - Run tests"
    @echo "  just clean      - Clean build artifacts"
    @echo "  just package    - Package application"
    @echo "  just fmt        - Format all code (TS + Rust)"
    @echo "  just fmt-check  - Check formatting without fixing"
    @echo "  just lint       - Run all linters"
    @echo "  just lint-fix   - Fix linting issues"
    @echo "  just type-check - Run type checks (TS + Rust)"
    @echo "  just check      - Complete code quality check"
    @echo "  just info       - Show this help"
    @echo "  just watch      - Watch for changes and auto-rebuild"

# Watch for changes and rebuild
watch:
    @echo "Watching for file changes..."
    @echo "Use Ctrl+C to stop"
    @while true; do
    find src/ src-tauri/src/ -name "*.rs" -o -name "*.ts" -o -name "*.tsx" | \
        entr -d -r just dev
    done
