#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

if ! command -v flox >/dev/null 2>&1; then
  echo "error: flox is not installed or not on PATH" >&2
  exit 1
fi

if [[ $# -eq 0 ]]; then
  exec flox activate --dir "$ROOT_DIR"
fi

exec flox activate --dir "$ROOT_DIR" -- "$@"
