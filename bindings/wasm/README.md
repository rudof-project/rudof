# rudof_wasm

WebAssembly bindings for [rudof](https://github.com/rudof-project/rudof):
validate RDF data with **ShEx** or **SHACL** from JavaScript, in the browser or
in Node.js.

This is a first step towards running rudof on `wasm`. All inputs are passed as
strings, so anything that needs the filesystem or the network is not
supported: reading files, dereferencing IRIs, ShEx `IMPORT`s and remote SPARQL
endpoints. SPARQL queries over the data itself (SHACL-SPARQL constraints, the
SPARQL SHACL engine, ShapeMap query selectors) run on Oxigraph's embedded
store.

## Building

You need the `wasm32-unknown-unknown` target and a `wasm-bindgen-cli` whose
version matches the `wasm-bindgen` crate in `Cargo.lock`:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --locked --version <version of wasm-bindgen in Cargo.lock>
./build.sh
```

`build.sh` generates two packages:

- `pkg/`: an ES module for browsers and bundlers (`wasm-bindgen --target web`)
- `pkg-node/`: a CommonJS module for Node.js (`wasm-bindgen --target nodejs`)

If `wasm-opt` (from [binaryen](https://github.com/WebAssembly/binaryen)) is
installed, `build.sh` also uses it to shrink the `.wasm` files.

## API

Both functions return a JSON string and throw an `Error` if an input cannot be
parsed. The optional format arguments accept `turtle` (the default),
`ntriples`, `rdfxml`, `trig`, `n3`, `nquads` and `jsonld`. The optional
`base` is used to resolve relative IRIs.

```ts
validateShex(data: string, schema: string, shapemap: string,
             dataFormat?: string, base?: string): string
```

Validates `data` against a ShExC `schema` for the associations in `shapemap`
(ShapeMap compact syntax, e.g. `:alice@:Person, :bob@:Person`, or with query
selectors such as `{FOCUS :name _}@:Person` and `SPARQL "SELECT ..."@:Person`).
Returns
`{ conforms, results }`, where each result has the `node`, `shape`, `status`
(`conformant` or `nonconformant`), `reason` and `appInfo` of one association.

```ts
validateShacl(data: string, shapes: string,
              dataFormat?: string, shapesFormat?: string, base?: string,
              mode?: "native" | "sparql"): string
```

Validates `data` against a SHACL shapes graph. `mode` selects the engine that
evaluates the core constraints: `native` (the default) or `sparql`.
SHACL-SPARQL constraints (`sh:sparql`) are supported in both modes. Returns
`{ conforms, results }`, where each result has the `focusNode`, `path`,
`value`, `sourceShape`, `constraintComponent`, `severity` and `messages`
(a list of `{ text, lang }`) of one validation result.

## Examples

- Node.js: `node examples/node.cjs`
- Browser: serve this directory over HTTP (e.g. `python3 -m http.server`) and
  open `examples/index.html`.

```js
const { validateShex } = require("./pkg-node/rudof_wasm.js");

const result = JSON.parse(validateShex(
  `prefix : <http://example.org/>
   :alice :name "Alice" .`,
  `prefix : <http://example.org/>
   prefix xsd: <http://www.w3.org/2001/XMLSchema#>
   :Person { :name xsd:string }`,
  ":alice@:Person",
));
console.log(result.conforms); // true
```

## Testing

The tests in `tests/` run natively and on `wasm`:

```sh
cargo test -p rudof_wasm
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  cargo test -p rudof_wasm --target wasm32-unknown-unknown
```
