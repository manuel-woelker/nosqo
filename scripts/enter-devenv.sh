#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if ! command -v devenv >/dev/null 2>&1; then
  echo "error: devenv is not installed or not on PATH" >&2
  exit 1
fi

cd "$ROOT_DIR"

if [[ $# -eq 0 ]]; then
  exec devenv shell --clean TERM,COLORTERM,DISPLAY,WAYLAND_DISPLAY,XAUTHORITY,SSH_AUTH_SOCK
fi

exec devenv shell --clean TERM,COLORTERM,DISPLAY,WAYLAND_DISPLAY,XAUTHORITY,SSH_AUTH_SOCK -- "$@"
