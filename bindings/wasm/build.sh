#!/usr/bin/env bash
# Builds the rudof_wasm module and generates its JavaScript bindings:
#   pkg/       ES module for browsers and bundlers (wasm-bindgen --target web)
#   pkg-node/  CommonJS module for Node.js (wasm-bindgen --target nodejs)
#
# Requires the wasm32-unknown-unknown target and a wasm-bindgen-cli whose
# version matches the `wasm-bindgen` crate in Cargo.lock:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version <version in Cargo.lock> --locked
set -euo pipefail

cd "$(dirname "$0")"
root="$(cargo metadata --format-version 1 --no-deps | sed -n 's/.*"workspace_root":"\([^"]*\)".*/\1/p')"

# All the features of rudof_wasm by default. RUDOF_WASM_FEATURES selects some
# instead, e.g. RUDOF_WASM_FEATURES="" for only RDF data, ShEx, SHACL and
# SPARQL, or RUDOF_WASM_FEATURES="dctap,conversion" (see the README).
features=()
if [ -n "${RUDOF_WASM_FEATURES+set}" ]; then
    features=(--no-default-features --features "$RUDOF_WASM_FEATURES")
fi

# The `wasm-release` profile (see the workspace Cargo.toml) optimizes for size.
cargo build -p rudof_wasm --profile wasm-release --target wasm32-unknown-unknown "${features[@]}"
wasm="$root/target/wasm32-unknown-unknown/wasm-release/rudof_wasm.wasm"
wasm-bindgen --target web --out-dir pkg "$wasm"
wasm-bindgen --target nodejs --out-dir pkg-node "$wasm"

if command -v wasm-opt > /dev/null; then
    for f in pkg/rudof_wasm_bg.wasm pkg-node/rudof_wasm_bg.wasm; do
        wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int -o "$f" "$f"
    done
fi
