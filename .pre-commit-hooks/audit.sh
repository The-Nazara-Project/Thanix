#!/usr/bin/env bash
set -euo pipefail

echo "[pre-commit (Thanix)] Checking for cargo-audit..."

if ! command -v cargo-audit >/dev/null 2>&1; then
    echo "[pre-commit (Thanix)] cargo-audit not found; installing..."
    cargo install cargo-audit --locked
else
    echo "[pre-commit (Thanix)] cargo-audit already installed."
fi

cargo audit