#!/usr/bin/env bash
set -euo pipefail

echo '== NVIDIA driver visible inside the container =='
nvidia-smi

echo
echo '== llama.cpp CUDA devices (run build-llama-cpp.sh first) =='
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/build/llama.cpp/bin/llama-cli"
if [[ -x "$BIN" ]]; then
  "$BIN" --list-devices
else
  echo "llama.cpp is not built yet. Run: ./scripts/build-llama-cpp.sh"
fi
