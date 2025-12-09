#!/bin/bash
# Build script for Media Dashboard C++ Application

set -e

echo "🔨 Building Media Dashboard C++ Application..."
echo ""

# Create build directory
mkdir -p build
cd build

# Configure with CMake
echo "⚙️  Configuring CMake..."
cmake -DCMAKE_BUILD_TYPE=Release ..

# Build
echo "🏗️  Compiling..."
cmake --build . --config Release

# Show result
echo ""
echo "✅ Build complete!"
echo ""
echo "Executable location: build/bin/media-dashboard"
echo ""
echo "Run with: ./build/bin/media-dashboard"
