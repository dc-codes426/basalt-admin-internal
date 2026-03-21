#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLIENTS_DIR="$(cd "$SCRIPT_DIR/../generated-apiclients" && pwd)"
DEST="$SCRIPT_DIR/generated/api-clients"

CLIENTS=(
  vultiserver
  networking-internal
)

rm -rf "$DEST"
mkdir -p "$DEST"

for client in "${CLIENTS[@]}"; do
  src="$CLIENTS_DIR/$client"
  if [[ ! -d "$src" ]]; then
    echo "ERROR: $src not found. Run generated-apiclients/generate.sh first."
    exit 1
  fi
  echo "Copying $client ..."
  cp -r "$src" "$DEST/$client"
done

echo "Done. Clients available at $DEST/"
ls -d "$DEST"/*/
