#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TEST_REPORT_FILE="$(mktemp)"
trap 'rm -f "$TEST_REPORT_FILE"' EXIT

pnpm --dir "$ROOT_DIR/ui" exec vitest run \
  --reporter=default \
  --reporter=json \
  --outputFile.json="$TEST_REPORT_FILE"

EXECUTED_TESTS="$(
  sed -En 's/.*"numTotalTests":([0-9]+).*/\1/p' "$TEST_REPORT_FILE" \
    | head -n 1
)"

echo "Task outcome: Executed ${EXECUTED_TESTS:-0} UI tests"
