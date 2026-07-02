#!/usr/bin/env bash
set -euo pipefail

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile default
fi

if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck source=/dev/null
  source "$HOME/.cargo/env"
fi

rustup toolchain install stable --component rust-src rustfmt clippy rust-analyzer
rustup target add riscv32imc-unknown-none-elf

if ! command -v espflash >/dev/null 2>&1; then
  cargo install espflash --locked
fi

if ! command -v esp-generate >/dev/null 2>&1; then
  cargo install esp-generate --locked
fi

rustc --version
cargo --version
espflash --version
esp-generate --version
