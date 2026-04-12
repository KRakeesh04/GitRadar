#!/usr/bin/env bash

set -e

echo "🧪 Running GitRadar tests..."

# Run TypeScript type checking
echo "🔍 Running TypeScript type check..."
bun run check

# Run frontend tests if they exist
if [ -f "package.json" ] && grep -q "test" package.json; then
    echo "📋 Running frontend tests..."
    bun test
else
    echo "ℹ️  No frontend tests configured"
fi

# Run Rust tests if src-tauri exists
if [ -d "src-tauri" ]; then
    echo "🦀 Running Rust backend tests..."
    cd src-tauri
    cargo test
    cd ..
fi

echo "✅ All tests completed!"
