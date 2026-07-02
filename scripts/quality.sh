#!/usr/bin/env bash
set -euo pipefail

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
fi

cargo fmt --all --check
taplo fmt --check Cargo.toml .cargo/config.toml rust-toolchain.toml deny.toml
cargo clippy --all-targets -- -D warnings
cargo machete
cargo audit
cargo deny check bans licenses sources
