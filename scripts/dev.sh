#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
STATE_DIR="$ROOT_DIR/.devcontainer/state"
USER_DATA_DIR="$STATE_DIR/devcontainer-user-data"
FINGERPRINT_FILE="$STATE_DIR/devcontainer.fingerprint"
REMOVE_EXISTING_CONTAINER=false
declare -a UP_ARGS=()

compute_fingerprint() {
  sha256sum \
    "$ROOT_DIR/.devcontainer/devcontainer.json" \
    "$ROOT_DIR/.devcontainer/Dockerfile" \
    "$ROOT_DIR/.devcontainer/post-create.sh" \
    | sha256sum \
    | awk '{ print $1 }'
}

if [[ "${1:-}" == "--help" ]]; then
  cat <<'EOF'
Usage: scripts/dev.sh [--rebuild] [command...]

Starts the repo devcontainer and runs a command inside it.
Without a command, opens an interactive Bash shell.

Options:
  --rebuild  Force removal of any existing devcontainer before startup.
EOF
  exit 0
fi

if [[ "${1:-}" == "--rebuild" ]]; then
  REMOVE_EXISTING_CONTAINER=true
  shift
fi

if ! command -v devcontainer >/dev/null 2>&1; then
  echo "error: devcontainer is not installed or not on PATH" >&2
  exit 1
fi

mkdir -p \
  "$STATE_DIR/cargo-registry" \
  "$STATE_DIR/cargo-git" \
  "$STATE_DIR/cargo-target" \
  "$USER_DATA_DIR"

CURRENT_FINGERPRINT="$(compute_fingerprint)"
PREVIOUS_FINGERPRINT=""

if [[ -f "$FINGERPRINT_FILE" ]]; then
  PREVIOUS_FINGERPRINT="$(<"$FINGERPRINT_FILE")"
fi

if [[ "$CURRENT_FINGERPRINT" != "$PREVIOUS_FINGERPRINT" ]]; then
  REMOVE_EXISTING_CONTAINER=true
fi

if [[ "$REMOVE_EXISTING_CONTAINER" == true ]]; then
  UP_ARGS+=(--remove-existing-container)
fi

devcontainer up \
  --workspace-folder "$ROOT_DIR" \
  --user-data-folder "$USER_DATA_DIR" \
  "${UP_ARGS[@]}" \
  >/dev/null

printf '%s\n' "$CURRENT_FINGERPRINT" >"$FINGERPRINT_FILE"

if [[ $# -eq 0 ]]; then
  exec devcontainer exec \
    --workspace-folder "$ROOT_DIR" \
    --user-data-folder "$USER_DATA_DIR" \
    bash -l
fi

exec devcontainer exec \
  --workspace-folder "$ROOT_DIR" \
  --user-data-folder "$USER_DATA_DIR" \
  "$@"
