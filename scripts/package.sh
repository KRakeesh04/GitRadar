#!/usr/bin/env bash

set -e

echo "📦 Packaging GitRadar..."

bun run build
bun run tauri build

echo "📁 Output located in:"
echo "src-tauri/target/release/bundle/"