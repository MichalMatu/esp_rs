#!/usr/bin/env bash
set -euo pipefail

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
fi

cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo build --release --bins
