#!/usr/bin/env bash

set -e

echo "🔧 Setting up GitRadar development environment..."

# Check if Bun is installed
if ! command -v bun &> /dev/null; then
    echo "❌ Bun is not installed. Please install Bun first:"
    echo "   curl -fsSL https://bun.sh/install | bash"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Install Node.js dependencies
echo "📦 Installing Node.js dependencies..."
bun install

# Install Tauri CLI if not already installed
echo "🦀 Installing Tauri CLI..."
bun add -D @tauri-apps/cli@latest

# Check if Tauri dependencies are installed
echo "🔍 Checking Tauri system dependencies..."
if command -v tauri &> /dev/null; then
    echo "✅ Running Tauri dependency check..."
    bun run tauri info
else
    echo "⚠️  Tauri CLI not found in PATH, using local version"
fi

echo "✅ Setup complete! You can now run:"
echo "   ./scripts/dev.sh    # Start development server"
echo "   ./scripts/build.sh  # Build for production"
echo "   ./scripts/test.sh   # Run tests"
