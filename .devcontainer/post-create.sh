#!/usr/bin/env bash

set -euo pipefail

rustc --version
cargo --version
cargo nextest --version
codex --version

cargo fetch
