#!/usr/bin/env bash
# Builds the rudof_wasm module and assembles the `@rudof/rudof` npm package in pkg/:
#   pkg/package.json  package metadata (from npm/package.json, with the
#                     version of the rudof_wasm crate)
#   pkg/web/          ES module for browsers and bundlers (wasm-bindgen --target web)
#   pkg/node/         CommonJS module for Node.js (wasm-bindgen --target nodejs)
#   pkg/endpoints/    the same two modules (web/ and node/), built with the
#                     `endpoints` feature (remote SPARQL endpoints), which
#                     only load in runtimes with JSPI
#
# Both targets of a build load the same web/rudof_wasm_bg.wasm.
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
# The endpoints/ build adds remote SPARQL endpoints, which need JSPI (see the
# README), to all the other features.
endpoints_features=(--no-default-features --features "comparison,conversion,dctap,pgschema,rdf-config,endpoints")

# Builds rudof_wasm with the cargo arguments after the first, and writes its
# web and Node.js modules to <dir>/web and <dir>/node.
build_module() {
    local dir="$1"
    shift
    # The `wasm-release` profile (see the workspace Cargo.toml) optimizes for size.
    cargo build -p rudof_wasm --profile wasm-release --target wasm32-unknown-unknown "$@"
    local wasm="$root/target/wasm32-unknown-unknown/wasm-release/rudof_wasm.wasm"

    wasm-bindgen --target web --out-dir "$dir/web" "$wasm"
    wasm-bindgen --target nodejs --out-dir "$dir/node" "$wasm"

    # Both targets generate the same .wasm, so the Node.js module loads the one in
    # web/ instead of shipping a second copy.
    local node_js="$dir/node/rudof_wasm.js"
    if ! grep -q '`${__dirname}/rudof_wasm_bg.wasm`' "$node_js"; then
        echo "Unexpected wasm-bindgen output: $node_js does not load \${__dirname}/rudof_wasm_bg.wasm" >&2
        exit 1
    fi
    sed -i.bak 's|`${__dirname}/rudof_wasm_bg.wasm`|`${__dirname}/../web/rudof_wasm_bg.wasm`|' "$node_js"
    rm "$node_js.bak" "$dir/node/rudof_wasm_bg.wasm" "$dir/node/rudof_wasm_bg.wasm.d.ts"

    # wasm-bindgen declares the methods of `#[wasm_bindgen(jspi)]` exports (the
    # `endpoints` feature) as `async` in the `.d.ts` files, which TypeScript
    # rejects in declarations (TS1040); their `Promise` return type says enough.
    local dts
    for dts in "$dir/web/rudof_wasm.d.ts" "$dir/node/rudof_wasm.d.ts"; do
        sed -i.bak -E 's/^([[:space:]]+)async ([A-Za-z_$][A-Za-z0-9_$]*\()/\1\2/' "$dts"
        rm "$dts.bak"
    done

    if command -v wasm-opt > /dev/null; then
        # JSPI modules (the `endpoints` feature) use exception tags and
        # reference types, which wasm-opt rejects unless they are enabled.
        local opt_features=(--enable-bulk-memory --enable-nontrapping-float-to-int)
        if grep -q 'WebAssembly.Suspending' "$dir/web/rudof_wasm.js"; then
            opt_features+=(--enable-exception-handling --enable-reference-types)
        fi
        wasm-opt -Oz "${opt_features[@]}" \
            -o "$dir/web/rudof_wasm_bg.wasm" "$dir/web/rudof_wasm_bg.wasm"
    elif [ "${RUDOF_WASM_OPT:-}" = "required" ]; then
        echo "wasm-opt not found (RUDOF_WASM_OPT=required)" >&2
        exit 1
    else
        echo "wasm-opt not found: the .wasm is not optimized further" >&2
    fi

    # The web glue is an ES module, the Node.js one CommonJS.
    echo '{ "type": "module" }' > "$dir/web/package.json"
    echo '{ "type": "commonjs" }' > "$dir/node/package.json"
}

rm -rf pkg
build_module pkg "${features[@]}"
build_module pkg/endpoints "${endpoints_features[@]}"

sed "s/\"version\": \"0.0.0\"/\"version\": \"$version\"/" npm/package.json > pkg/package.json
cp npm/README.md pkg/README.md
cp "$root/LICENSE-MIT" "$root/LICENSE-APACHE" pkg/
echo "Built the @rudof/rudof npm package $version in $(pwd)/pkg"
