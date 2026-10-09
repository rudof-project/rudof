# rudof_wasm

WebAssembly bindings for [rudof](https://github.com/rudof-project/rudof):
validate RDF data with **ShEx** or **SHACL**, and query it with **SPARQL**,
from JavaScript, in the browser or in Node.js.

They are published to npm as [`@rudof/rudof`](https://www.npmjs.com/package/@rudof/rudof)
(`npm install @rudof/rudof`); [npm/README.md](npm/README.md) is the package's README,
with its usage. This file is about developing the bindings.

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

`build.sh` assembles the npm package in `pkg/`:

- `pkg/package.json`: [npm/package.json](npm/package.json), with the version of
  the workspace
- `pkg/web/`: an ES module for browsers and bundlers (`wasm-bindgen --target web`)
- `pkg/node/`: a CommonJS module for Node.js (`wasm-bindgen --target nodejs`),
  which loads the same `.wasm` file as `pkg/web/`

`import "@rudof/rudof"` and `require("@rudof/rudof")` pick `node/` in Node.js and `web/`
everywhere else (`exports` in `package.json`); `@rudof/rudof/web` and `@rudof/rudof/node`
select one explicitly.

`build.sh` builds with the `wasm-release` profile of the workspace, which
optimizes for size (`opt-level = "z"`, LTO, one codegen unit). If `wasm-opt`
(from [binaryen](https://github.com/WebAssembly/binaryen), e.g.
`npm install -g binaryen`) is installed, it also uses it to shrink the `.wasm`
by about 40%; releases require it (`RUDOF_WASM_OPT=required`).

`npm/smoke-test.sh` packs `pkg/` with `npm pack`, installs the tarball in an
empty project and uses it with `require`, `import` and the web build, as CI
does. `examples/node.cjs` (`node examples/node.cjs`) uses `pkg/` directly, and
`examples/index.html` uses it in a browser (serve this directory over HTTP,
e.g. with `python3 -m http.server`, and open `/examples/index.html`).

## Publishing

The [npm workflow](../../.github/workflows/npm.yml) publishes the package when
a GitHub release is published (see `release.yml`), with the version of the
workspace; release candidates (`X.Y.Z-rc.N`) get the `next` tag instead of
`latest`. It can also be run by hand, as a dry run by default.

### Smaller builds

RDF data, ShEx, SHACL and SPARQL are always included. Other parts of the API
are Cargo features, all enabled by default, which can be left out to get a
smaller `.wasm`:

| Feature | Methods |
|---------|---------|
| `conversion` | `convertSchemas`, and UML output in `serializeCurrentShex` |
| `comparison` | `compareSchemas` |
| `dctap` | `readDctap`, `serializeDctap`, `resetDctap` |
| `pgschema` | `readPgschema`, `serializePgschema`, `readTypemap`, `validatePgschema`, `serializePgschemaValidationResults` and their resets, and property graph data (`readData(data, "pg")`) |
| `rdf-config` | `readRdfConfig`, `serializeRdfConfig`, `resetRdfConfig` |

`RUDOF_WASM_FEATURES` selects the features that `build.sh` builds with:

```sh
RUDOF_WASM_FEATURES="" ./build.sh                 # none: data, ShEx, SHACL, SPARQL
RUDOF_WASM_FEATURES="conversion,dctap" ./build.sh # only these
```

Methods of features that are left out are not generated, so they are missing
from the `.d.ts` files too.

Sizes of the `.wasm` built by `build.sh` before `wasm-opt`, which shrinks it
by about 40% more (to 6.0 MB, 2.2 MB gzipped, with every feature):

| Features | Size | Gzipped |
|----------|------|---------|
| all (default) | 9.5 MB | 2.7 MB |
| none | 7.4 MB | 2.1 MB |
| only `conversion` | 8.5 MB | 2.4 MB |
| only `pgschema` | 8.1 MB | 2.3 MB |
| only `dctap`, `rdf-config` or `comparison` | 7.5 MB | 2.1 MB |

The features are those of `rudof_lib`, which can be used in the same way by
other crates.

## API

The bindings are built on `rudof_lib` and mirror the
[Python bindings](../python): a `Rudof` class keeps a session (loaded data,
schemas, queries and results), and `validateShex`/`validateShacl` do a whole
validation in one call. Names follow JavaScript conventions (`read_shex`
becomes `readShex`), and the generated `.d.ts` files declare every type.

```js
import init, { Rudof, RudofConfig } from "@rudof/rudof"; // or "./pkg/web/rudof_wasm.js"
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
| RDF data | `readData(data, format?, base?, readerMode?, merge?)`, `serializeData(format?)`, `addTriple(subject, predicate, object, strictIris?)`, `removeTriple(subject, predicate, object, strictIris?)`, `triples(subject?, predicate?, object?, strictIris?, limit?)`, `dereference(uri, readerMode?, merge?)`, `listEndpoints()` |
| Nodes | `nodeInfo(nodeSelector, predicates?, mode?, showColors?, depth?)`, `nodeNeighborhood(nodeSelector, predicates?, mode?, depth?, strictIris?, limit?)` |
| ShEx | `readShex(schema, format?, base?, readerMode?)`, `checkShex(schema, format?, base?)`, `serializeCurrentShex(format?, shapeLabel?)`, `readShapemap(shapemap, format?, baseNodes?, baseShapes?)`, `serializeShapemap(format?)`, `validateShex()`, `serializeShexValidationResults(format?, sortMode?)`, `materialize(format?, node?)`, `addExternalResolver(spec)`, `clearExternalResolvers()`, `Rudof.listExternalResolvers()` |
| SHACL | `readShacl(shapes?, format?, base?, readerMode?)` (without `shapes`, they are taken from the data), `serializeShacl(format?)`, `validateShacl(mode?)`, `serializeShaclValidationResults(format?, sortMode?)` |
| SPARQL | `readQuery(query, queryType?)`, `runQuery()`, `serializeQueryResults(format?)` |
| Schemas | `convertSchemas(schema, inputMode, outputMode, inputFormat, outputFormat, base?, readerMode?, shape?)`, `compareSchemas(schema1, schema2, mode1, mode2, format1, format2, base1?, base2?, label1?, label2?, readerMode?)` |
| DCTAP | `readDctap(dctap, format?)`, `serializeDctap(format?)` |
| rdf-config | `readRdfConfig(rdfConfig, format?)`, `serializeRdfConfig(format?)` |
| Service descriptions | `readServiceDescription(serviceDescription, format?, base?, readerMode?)`, `serializeServiceDescription(format?)` |
| Property graph schemas | `readPgschema(pgschema, format?)`, `serializePgschema(format?)`, `readTypemap(typemap)`, `validatePgschema()` (with property graph data loaded with `readData(data, "pg")`), `serializePgschemaValidationResults(format?)` |
| Prefixes | `prefixes()`, `addPrefix(alias, iri)`, `removePrefix(alias)`, `renamePrefix(old, new)`, `copyPrefix(old, new)` |
| Resets | `resetAll()`, `resetData()`, `resetShex()`, `resetShexSchema()`, `resetShapemap()`, `resetShacl()`, `resetShaclValidation()`, `resetQuery()`, `resetQueryResults()`, `resetDctap()`, `resetRdfConfig()`, `resetServiceDescription()`, `resetPgschema()`, `resetTypemap()`, `resetPgschemaValidation()`, `resetValidationResults()` |

Everything in the Python bindings is available, except what needs files or
native libraries: `read_map_state`, `compile_shex_to_file` and
`read_shex_precompiled` (which take file paths), property graph databases
(`connect_pg_db`, `pg_db_ddl`, `load_pg_db`, `query_cypher`) and data
generation. `dereference` and `listEndpoints` are there for completeness, but
there is no network access on wasm: `dereference` throws a `DataError`, and
`listEndpoints` returns `[]`. Conversions that render images or write HTML to a
folder fail too.

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
- `PgSchemaValidationReport`: `{ conforms, entries, violations }`, where each
  entry has the `nodeId`, `typeName`, `conforms` and `details` of one
  node/type association.
- `NodeNeighborhood`: `{ arcs, truncated }`, where each arc has its `root`,
  `direction` (`outgoing` or `incoming`), `depth`, `node`, `predicate`,
  `neighbor` and `isLast`.
- `checkShex` returns `{ valid, message }`, and `listExternalResolvers`
  a list of `{ name, description, specSyntax }`.

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
