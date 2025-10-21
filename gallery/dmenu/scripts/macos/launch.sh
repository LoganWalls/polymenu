#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &> /dev/null && pwd)
CACHE_DIR="${CACHE_DIR:-$HOME/.cache/polymenu/app-launcher}"
INDEX="$CACHE_DIR/index.json"

if [ ! -f "$INDEX" ]; then
  echo "No existing cache found"
  nu "$SCRIPT_DIR/build-cache.nu" "$CACHE_DIR"
fi

polymenu --config "$SCRIPT_DIR/../../config.toml" \
  --mount "icons:$HOME/$CACHE_DIR/icons" \
  --file "$INDEX" \
  | xargs -l open -b
