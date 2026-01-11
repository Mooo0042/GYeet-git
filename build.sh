#!/bin/bash
# Build script for GYeet

set -e

echo "🚀 Building GYeet..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install it from https://rustup.rs/"
    exit 1
fi

# Check if GTK4 is installed
if ! pkg-config --exists gtk4 2>/dev/null; then
    echo "⚠️  Warning: GTK4 might not be installed."
    echo "   On Debian/Ubuntu: sudo apt install libgtk-4-dev build-essential"
    echo "   On Arch: sudo pacman -S gtk4"
    echo "   On Fedora: sudo dnf install gtk4-devel"
fi

# Build in release mode
echo "📦 Compiling..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo "📍 Binary location: target/release/gyeet"
    echo ""
    echo "To run: ./target/release/gyeet"
else
    echo "❌ Build failed!"
    exit 1
fi

