#!/usr/bin/env bash
# Builds the rudof_wasm module and assembles the `@rudof/rudof` npm package in pkg/:
#   pkg/package.json  package metadata (from npm/package.json, with the
#                     version of the rudof_wasm crate)
#   pkg/web/          ES module for browsers and bundlers (wasm-bindgen --target web)
#   pkg/node/         CommonJS module for Node.js (wasm-bindgen --target nodejs)
#
# Both targets load the same pkg/web/rudof_wasm_bg.wasm.
#
# Requires the wasm32-unknown-unknown target and a wasm-bindgen-cli whose
# version matches the `wasm-bindgen` crate in Cargo.lock:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version <version in Cargo.lock> --locked
# If wasm-opt (from binaryen) is installed, it is used to shrink the module.
# Set RUDOF_WASM_OPT=required to fail when it is missing (as releases do).
set -euo pipefail

cd "$(dirname "$0")"
metadata="$(cargo metadata --format-version 1 --no-deps)"
root="$(sed -n 's/.*"workspace_root":"\([^"]*\)".*/\1/p' <<< "$metadata")"
version="$(grep -o '"name":"rudof_wasm","version":"[^"]*"' <<< "$metadata" | sed 's/.*"version":"\([^"]*\)"/\1/')"
if [ -z "$version" ]; then
    echo "Could not find the version of rudof_wasm" >&2
    exit 1
fi

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

rm -rf pkg
wasm-bindgen --target web --out-dir pkg/web "$wasm"
wasm-bindgen --target nodejs --out-dir pkg/node "$wasm"

# Both targets generate the same .wasm, so the Node.js module loads the one in
# web/ instead of shipping a second copy.
node_js=pkg/node/rudof_wasm.js
if ! grep -q '`${__dirname}/rudof_wasm_bg.wasm`' "$node_js"; then
    echo "Unexpected wasm-bindgen output: $node_js does not load \${__dirname}/rudof_wasm_bg.wasm" >&2
    exit 1
fi
sed -i.bak 's|`${__dirname}/rudof_wasm_bg.wasm`|`${__dirname}/../web/rudof_wasm_bg.wasm`|' "$node_js"
rm "$node_js.bak" pkg/node/rudof_wasm_bg.wasm pkg/node/rudof_wasm_bg.wasm.d.ts

if command -v wasm-opt > /dev/null; then
    wasm-opt -Oz --enable-bulk-memory --enable-nontrapping-float-to-int \
        -o pkg/web/rudof_wasm_bg.wasm pkg/web/rudof_wasm_bg.wasm
elif [ "${RUDOF_WASM_OPT:-}" = "required" ]; then
    echo "wasm-opt not found (RUDOF_WASM_OPT=required)" >&2
    exit 1
else
    echo "wasm-opt not found: the .wasm is not optimized further" >&2
fi

# The web glue is an ES module, the Node.js one CommonJS.
echo '{ "type": "module" }' > pkg/web/package.json
echo '{ "type": "commonjs" }' > pkg/node/package.json

sed "s/\"version\": \"0.0.0\"/\"version\": \"$version\"/" npm/package.json > pkg/package.json
cp npm/README.md pkg/README.md
cp "$root/LICENSE-MIT" "$root/LICENSE-APACHE" pkg/
echo "Built the @rudof/rudof npm package $version in $(pwd)/pkg"
