#!/usr/bin/env bash

set -euo pipefail

echo
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST_PATH="$ROOT_DIR/Cargo.toml"
GREEN=$'\033[32m'
RESET=$'\033[0m'

now_ns() {
  date +%s%N
}

format_duration() {
  local duration_ns="$1"
  local duration_ms=$((duration_ns / 1000000))
  local seconds=$((duration_ms / 1000))
  local millis=$((duration_ms % 1000))
  printf "%d.%03ds" "$seconds" "$millis"
}

print_timing_line() {
  local emoji="$1"
  local label="$2"
  local elapsed_ns="$3"
  printf "%s %-28s %8s\n" "$emoji" "$label" "$(format_duration "$elapsed_ns")"
}

run_step() {
  local name="$1"
  shift

  local start_ns
  start_ns="$(now_ns)"
  "$@"
  local end_ns
  end_ns="$(now_ns)"

  local elapsed_ns=$((end_ns - start_ns))
  local emoji="⏱️"
  case "$name" in
    "cargo fmt")
      emoji="🧹"
      ;;
    "cargo build")
      emoji="🔨"
      ;;
    "cargo clippy")
      emoji="📎"
      ;;
  esac
  print_timing_line "$emoji" "Running $name..." "$elapsed_ns"
}

if [[ ! -f "$MANIFEST_PATH" ]]; then
  echo "❌ error: workspace manifest not found at $MANIFEST_PATH" >&2
  exit 1
fi

TOTAL_START_NS="$(now_ns)"

run_step "cargo fmt" cargo fmt --manifest-path "$MANIFEST_PATH" --all --quiet

run_step "cargo build" cargo build --manifest-path "$MANIFEST_PATH" --workspace --all-targets --all-features --quiet

run_step "cargo clippy" cargo clippy --manifest-path "$MANIFEST_PATH" --workspace --all-targets --all-features --quiet --keep-going --no-deps -- -D warnings

TEST_START_NS="$(now_ns)"
TEST_OUTPUT_FILE="$(mktemp)"
trap 'rm -f "$TEST_OUTPUT_FILE"' EXIT

if ! cargo nextest run --manifest-path "$MANIFEST_PATH" --workspace --all-targets --all-features --show-progress=none --status-level fail --no-fail-fast >"$TEST_OUTPUT_FILE" 2>&1; then
  cat "$TEST_OUTPUT_FILE"
  exit 1
fi

EXECUTED_TESTS="$(
  awk '
    /Summary .* tests run:/ {
      for (i = 1; i <= NF; i++) {
        if ($i == "tests" && $(i + 1) == "run:") {
          total += $(i - 1)
        }
      }
    }
    END {
      print total + 0
    }
  ' "$TEST_OUTPUT_FILE"
)"

TEST_END_NS="$(now_ns)"
print_timing_line "🧪" "Running cargo nextest..." "$((TEST_END_NS - TEST_START_NS))"

TOTAL_END_NS="$(now_ns)"
print_timing_line "🏁" "Complete check..." "$((TOTAL_END_NS - TOTAL_START_NS))"

printf "🎉 All checks passed - %s%s%s tests\n" "$GREEN" "$EXECUTED_TESTS" "$RESET"
