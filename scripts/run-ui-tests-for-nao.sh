#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEST_OUTPUT_FILE="$(mktemp)"
trap 'rm -f "$TEST_OUTPUT_FILE"' EXIT

if ! pnpm --dir "$ROOT_DIR/ui" run test:run >"$TEST_OUTPUT_FILE" 2>&1; then
  cat "$TEST_OUTPUT_FILE"
  exit 1
fi

EXECUTED_TESTS="$(
  grep -E '^[[:space:]]*Tests[[:space:]]+' "$TEST_OUTPUT_FILE" \
    | sed -E 's/.*\(([0-9]+)\).*/\1/' \
    | tail -n 1
)"

echo "Task outcome: Executed ${EXECUTED_TESTS:-0} UI tests"
