#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
THIRD_PARTY="$ROOT/third_party"
SRC="$THIRD_PARTY/llama.cpp"
BUILD="$ROOT/build/llama.cpp"

mkdir -p "$THIRD_PARTY" "$ROOT/build" "$ROOT/models"

if [[ ! -d "$SRC/.git" ]]; then
  git clone --depth 1 https://github.com/ggml-org/llama.cpp.git "$SRC"
else
  echo "Found existing llama.cpp checkout at $SRC"
  echo "Update it manually with: git -C '$SRC' pull --ff-only"
fi

cmake -S "$SRC" -B "$BUILD" -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DGGML_CUDA=ON \
  -DGGML_NATIVE=OFF

cmake --build "$BUILD" --parallel "$(nproc)"
echo
echo "Build complete. Main executable: $BUILD/bin/llama-cli"
echo "Try: $BUILD/bin/llama-cli --list-devices"
