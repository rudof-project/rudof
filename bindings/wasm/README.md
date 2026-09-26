# rudof_wasm

WebAssembly bindings for [rudof](https://github.com/rudof-project/rudof):
validate RDF data with **ShEx** or **SHACL**, and query it with **SPARQL**,
from JavaScript, in the browser or in Node.js.

All inputs are passed as strings, so anything that needs the filesystem, the
network or native tools is not supported: reading files, dereferencing IRIs,
ShEx `IMPORT`s, remote SPARQL endpoints and rendering images. SPARQL queries
over the data itself (SPARQL queries, SHACL-SPARQL constraints, the SPARQL
SHACL engine, ShapeMap query selectors) run on Oxigraph's embedded store.

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

The bindings are built on `rudof_lib` and mirror the
[Python bindings](../python): a `Rudof` class keeps a session (loaded data,
schemas, queries and results), and `validateShex`/`validateShacl` do a whole
validation in one call. Names follow JavaScript conventions (`read_shex`
becomes `readShex`), and the generated `.d.ts` files declare every type.

```js
import init, { Rudof, RudofConfig } from "./pkg/rudof_wasm.js";
await init();

const rudof = new Rudof(RudofConfig.fromToml('base_iri = "http://example.org/"'));
rudof.readData(data);                     // Turtle by default
rudof.readShex(schema);                   // ShExC by default
rudof.readShapemap(":alice@:Person, {FOCUS :name _}@:Person");
const report = rudof.validateShex();      // { conforms, entries, violations }

rudof.readShacl(shapes);
const shaclReport = rudof.validateShacl("sparql");

rudof.readQuery("SELECT ?s WHERE { ?s ?p ?o }");
const results = rudof.runQuery();         // { kind: "select", variables, rows }
```

### Rudof

| Area | Methods |
|------|---------|
| Session | `new Rudof(config?)`, `updateConfig(config)`, `getVersion()` |
| RDF data | `readData(data, format?, base?, readerMode?, merge?)`, `serializeData(format?)` |
| ShEx | `readShex(schema, format?, base?, readerMode?)`, `serializeCurrentShex(format?, shapeLabel?)`, `readShapemap(shapemap, format?, baseNodes?, baseShapes?)`, `serializeShapemap(format?)`, `validateShex()`, `serializeShexValidationResults(format?, sortMode?)` |
| SHACL | `readShacl(shapes?, format?, base?, readerMode?)` (without `shapes`, they are taken from the data), `serializeShacl(format?)`, `validateShacl(mode?)`, `serializeShaclValidationResults(format?, sortMode?)` |
| SPARQL | `readQuery(query, queryType?)`, `runQuery()`, `serializeQueryResults(format?)` |
| Prefixes | `prefixes()`, `addPrefix(alias, iri)`, `removePrefix(alias)`, `renamePrefix(old, new)`, `copyPrefix(old, new)` |
| Resets | `resetAll()`, `resetData()`, `resetShex()`, `resetShexSchema()`, `resetShapemap()`, `resetShacl()`, `resetShaclValidation()`, `resetQuery()`, `resetQueryResults()` |

Inputs are strings. Formats and modes are strings too, with the names used by
the command line interface: e.g. `turtle`, `ntriples`, `rdfxml`, `trig`, `n3`,
`nquads` or `jsonld` for RDF, `shexc` or `shexj` for ShEx, `native` or `sparql`
for SHACL validation, `strict` or `lax` for the reader mode. Optional arguments
can be omitted or passed as `undefined`/`null`.

`new RudofConfig()` gives the default configuration, and
`RudofConfig.fromToml(toml)` reads one with the keys of `rudof.toml`. There is
no current directory to derive a base IRI from, so the default configuration
enables `auto_base`: relative IRIs are resolved against `http://base` unless a
base is given, either as an argument or with `base_iri` in the configuration.

### One-shot validation

```ts
validateShex(data: string, schema: string, shapemap: string,
             dataFormat?: string, base?: string): ShExValidationReport
validateShacl(data: string, shapes: string, dataFormat?: string,
              shapesFormat?: string, base?: string,
              mode?: "native" | "sparql"): ShaclValidationReport
```

### Results

Reports are plain objects, as in the Python bindings:

- `ShExValidationReport`: `{ conforms, entries, violations }`, where each entry
  has the `node`, `shape`, `status` (`conformant`, `nonconformant`, ...) and
  `details` of one association, and `violations` are the non-conformant ones.
- `ShaclValidationReport`: `{ conforms, entries, violations }`, where each entry
  has the `focusNode`, `path`, `value`, `sourceShape`, `constraintComponent`,
  `severity` and `messages` (a list of `{ text, lang }`) of one result.
  SHACL-SPARQL constraints (`sh:sparql`) are supported in both modes.
- `QueryResults`: `{ kind: "select", variables, rows }`,
  `{ kind: "ask", boolean }` or `{ kind: "graph", graph }`.

Missing values are `null`. Errors are thrown as `Error`s whose `name` is the
error category, matching the Python exception classes (`DataError`,
`ShExError`, `ShaclError`, `QueryError`, ...), or `RangeError` for an unknown
format or mode.

## Examples

- Node.js: `node examples/node.cjs`
- Browser: serve this directory over HTTP (e.g. `python3 -m http.server`) and
  open `examples/index.html`.

## Testing

The tests in `tests/` run natively and on `wasm` (`tests/js_api.rs`, which
checks the values and errors seen by JavaScript, only on `wasm`):

```sh
cargo test -p rudof_wasm
CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_RUNNER=wasm-bindgen-test-runner \
  cargo test -p rudof_wasm --target wasm32-unknown-unknown
```
