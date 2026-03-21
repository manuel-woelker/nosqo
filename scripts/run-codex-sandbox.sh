#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
HOST_HOME="${HOME:?HOME must be set}"
HOST_USER="${USER:-$(id -un)}"
HOST_CARGO_HOME="${CARGO_HOME:-$HOST_HOME/.cargo}"
HOST_RUSTUP_HOME="${RUSTUP_HOME:-$HOST_HOME/.rustup}"
SANDBOX_CWD="$ROOT_DIR"

if [[ "$(uname -s)" != "Linux" ]]; then
  echo "error: this script requires Linux because it uses bubblewrap" >&2
  exit 1
fi

if ! command -v bwrap >/dev/null 2>&1; then
  echo "error: bubblewrap (bwrap) is not installed" >&2
  exit 1
fi

if ! command -v codex >/dev/null 2>&1; then
  echo "error: codex is not installed or not on PATH" >&2
  exit 1
fi

if [[ "${1:-}" == "--help" ]]; then
  cat <<'EOF'
Usage: scripts/run-codex-sandbox.sh [codex args...]

Runs Codex inside a bubblewrap sandbox that:
- exposes the repository read-write
- exposes only selected paths from your home directory
- hides the rest of /home behind an empty tmpfs

Optional environment variables:
- NOSQO_CODEX_EXTRA_RO_BIND: colon-separated host paths to expose read-only
- NOSQO_CODEX_EXTRA_RW_BIND: colon-separated host paths to expose read-write
EOF
  exit 0
fi

if [[ "$PWD" == "$ROOT_DIR" ]] || [[ "$PWD" == "$ROOT_DIR/"* ]]; then
  SANDBOX_CWD="$PWD"
fi

CODEx_PATH="$(command -v codex)"
NODE_PATH="$(command -v node || true)"

declare -a BWRAP_ARGS=(
  --die-with-parent
  --new-session
  --proc /proc
  --dev /dev
  --tmpfs /tmp
  --tmpfs /var/tmp
  --tmpfs /home
)

declare -A CREATED_DIRS=()

ensure_dir() {
  local path="$1"
  if [[ "$path" == "/" ]]; then
    return
  fi
  if [[ -z "${CREATED_DIRS["$path"]+x}" ]]; then
    BWRAP_ARGS+=(--dir "$path")
    CREATED_DIRS["$path"]=1
  fi
}

ensure_dir_tree() {
  local path="$1"
  if [[ "$path" == "/" ]]; then
    return
  fi
  local parent
  parent="$(dirname "$path")"
  if [[ "$parent" != "$path" ]]; then
    ensure_dir_tree "$parent"
  fi
  ensure_dir "$path"
}

add_ro_bind() {
  local path="$1"
  if [[ -e "$path" ]]; then
    if [[ -d "$path" ]]; then
      ensure_dir_tree "$path"
    else
      ensure_dir_tree "$(dirname "$path")"
    fi
    BWRAP_ARGS+=(--ro-bind "$path" "$path")
  fi
}

add_rw_bind() {
  local path="$1"
  if [[ -e "$path" ]]; then
    if [[ -d "$path" ]]; then
      ensure_dir_tree "$path"
    else
      ensure_dir_tree "$(dirname "$path")"
    fi
    BWRAP_ARGS+=(--bind "$path" "$path")
  fi
}

add_colon_separated_binds() {
  local mode="$1"
  local raw="$2"
  local path
  IFS=':' read -r -a paths <<<"$raw"
  for path in "${paths[@]}"; do
    [[ -n "$path" ]] || continue
    if [[ "$mode" == "ro" ]]; then
      add_ro_bind "$path"
    else
      add_rw_bind "$path"
    fi
  done
}

for dir in /usr /bin /lib /lib64 /sbin /opt /etc /run/current-system/sw /nix/store; do
  add_ro_bind "$dir"
done

ensure_dir_tree "$HOST_HOME"
add_rw_bind "$ROOT_DIR"
add_rw_bind "$HOST_HOME/.codex"
add_rw_bind "$HOST_CARGO_HOME"
add_rw_bind "$HOST_RUSTUP_HOME"
add_ro_bind "$HOST_HOME/.gitconfig"
add_ro_bind "$HOST_HOME/.local/share/pnpm"
add_ro_bind "$CODEx_PATH"

if [[ -n "$NODE_PATH" ]]; then
  add_ro_bind "$NODE_PATH"
fi

if [[ -n "${NOSQO_CODEX_EXTRA_RO_BIND:-}" ]]; then
  add_colon_separated_binds "ro" "$NOSQO_CODEX_EXTRA_RO_BIND"
fi

if [[ -n "${NOSQO_CODEX_EXTRA_RW_BIND:-}" ]]; then
  add_colon_separated_binds "rw" "$NOSQO_CODEX_EXTRA_RW_BIND"
fi

declare -a ENV_ARGS=(
  --setenv HOME "$HOST_HOME"
  --setenv USER "$HOST_USER"
  --setenv LOGNAME "$HOST_USER"
  --setenv CARGO_HOME "$HOST_CARGO_HOME"
  --setenv RUSTUP_HOME "$HOST_RUSTUP_HOME"
  --setenv PATH "$HOST_CARGO_HOME/bin:/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin:/sbin"
)

for env_name in LANG LC_ALL TERM COLORTERM NO_COLOR FORCE_COLOR; do
  if [[ -n "${!env_name:-}" ]]; then
    ENV_ARGS+=(--setenv "$env_name" "${!env_name}")
  fi
done

exec bwrap \
  "${BWRAP_ARGS[@]}" \
  "${ENV_ARGS[@]}" \
  --chdir "$SANDBOX_CWD" \
  -- "$CODEx_PATH" --full-auto "$@"
