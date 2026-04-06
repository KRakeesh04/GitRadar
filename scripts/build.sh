#!/usr/bin/env bash

set -e

echo "📦 Building GitRadar..."

bun install
bun run build
bun run tauri build

echo "✅ Build completed!"