#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

SIZE="${1:-medium}"
ITERS="${2:-10}"

echo ">>> [1/3] Building SINGLE portable binary (default x86_64 target, no RUSTFLAGS)..."
cargo build --release --offline

echo ""
echo ">>> [2/3] Running the SAME portable binary across 5 Platform.toml profiles WITHOUT recompilation..."
for profile in scalar sse42 avx2 multicore_scalar multicore_avx2; do
    echo ""
    echo "=== Profile: $profile (PLATFORM_DESCRIPTION=$SCRIPT_DIR/profiles/$profile) ==="
    PLATFORM_DESCRIPTION="$SCRIPT_DIR/profiles/$profile" \
        ./target/release/micro-tensor-engine --size "$SIZE" --iters "$ITERS"
done

echo ""
echo ">>> [3/3] Building STATIC NATIVE binary (RUSTFLAGS=\"-C target-cpu=native\" --features static-multicore)..."
RUSTFLAGS="-C target-cpu=native" cargo build --release --offline \
    --features static-multicore \
    --target-dir target-static-native

echo ""
echo "=== Static Native vs Platform-Aware (same Ryzen/host hardware) ==="
PLATFORM_DESCRIPTION="$SCRIPT_DIR/profiles/multicore_avx2" \
    ./target-static-native/release/micro-tensor-engine --size "$SIZE" --iters "$ITERS"
