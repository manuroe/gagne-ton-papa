#!/bin/bash
# Script to download the ML model from the latest release of gagne-ton-papa-ml
# and copy ONNX Runtime WASM files
# This should be run before building or starting the web app

set -e

# Detect the script's directory and repo root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

MODEL_DIR="${1:-$REPO_ROOT/web/public/models}"
MODEL_FILENAME="gtp-quantized.onnx"

# Direct download URL from the latest release
# Update this URL when new releases are made
DOWNLOAD_URL="https://github.com/manuroe/gagne-ton-papa-ml/releases/download/0.0.0.1/gtp-quantized.onnx"

# Create target directory if it doesn't exist
mkdir -p "$MODEL_DIR"

# Download model if not exists
if [ -f "$MODEL_DIR/$MODEL_FILENAME" ]; then
    echo "Model already exists at $MODEL_DIR/$MODEL_FILENAME"
else
    echo "Downloading model from: $DOWNLOAD_URL"
    curl -L -o "$MODEL_DIR/$MODEL_FILENAME" "$DOWNLOAD_URL"

    if [ ! -f "$MODEL_DIR/$MODEL_FILENAME" ]; then
        echo "Error: Failed to download model"
        exit 1
    fi

    echo "Model downloaded successfully to $MODEL_DIR/$MODEL_FILENAME"
fi

# Copy ONNX Runtime WASM files to public folder
ORT_WASM_DIR="$REPO_ROOT/web/node_modules/onnxruntime-web/dist"
PUBLIC_DIR="$REPO_ROOT/web/public"

if [ -d "$ORT_WASM_DIR" ]; then
    echo "Copying ONNX Runtime WASM files to public folder..."
    cp -f "$ORT_WASM_DIR"/*.wasm "$PUBLIC_DIR/" 2>/dev/null || true
    cp -f "$ORT_WASM_DIR"/*.mjs "$PUBLIC_DIR/" 2>/dev/null || true
    echo "ONNX Runtime WASM files copied"
else
    echo "Warning: ONNX Runtime WASM files not found. Run npm install first."
fi
