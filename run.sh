#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"

MODE=debug
MODE_FLAG=()
if [[ "${1:-}" == "--release" ]]; then
  MODE=release
  MODE_FLAG=(--release)
elif [[ $# -gt 0 ]]; then
  echo "Bilinmeyen argüman: $1 (sadece --release destekleniyor)" >&2
fi

echo ">> ThemeGallery derleniyor ($MODE)..."
cargo build --workspace "${MODE_FLAG[@]}"

echo ">> çalıştırılıyor..."
exec cargo run -p ThemeGallery "${MODE_FLAG[@]}"
