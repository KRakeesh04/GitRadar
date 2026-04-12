#!/usr/bin/env bash

set -e

echo "🧹 Cleaning GitRadar build artifacts..."

# Clean frontend build artifacts
echo "📦 Cleaning frontend..."
rm -rf dist/
rm -rf node_modules/.vite

# Clean Tauri build artifacts
echo "🦀 Cleaning Tauri build..."
if [ -d "src-tauri" ]; then
    cd src-tauri
    cargo clean
    rm -rf target/
    cd ..
fi

# Clean logs and temporary files
echo "🗑️  Cleaning temporary files..."
find . -name "*.log" -delete 2>/dev/null || true
find . -name ".DS_Store" -delete 2>/dev/null || true

echo "✅ Cleanup complete!"
