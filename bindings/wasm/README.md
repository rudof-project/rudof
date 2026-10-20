# rudof_wasm

WebAssembly bindings for [rudof](https://github.com/rudof-project/rudof):
validate RDF data with **ShEx** or **SHACL**, and query it with **SPARQL**,
from JavaScript, in the browser or in Node.js.

They are published to npm as [`@rudof/rudof`](https://www.npmjs.com/package/@rudof/rudof)
(`npm install @rudof/rudof`); [npm/README.md](npm/README.md) is the package's README,
with its usage. This file is about developing the bindings.

All inputs are passed as strings, so anything that needs the filesystem, the
network or native tools is not supported: reading files, dereferencing IRIs,
ShEx `IMPORT`s and rendering images. SPARQL queries over the data itself
(SPARQL queries, SHACL-SPARQL constraints, the SPARQL SHACL engine, ShapeMap
query selectors) run on Oxigraph's embedded store. Remote SPARQL endpoints are
available in builds with the `endpoints` feature, which relies on JSPI, and
which the npm package ships as `@rudof/rudof/endpoints` (see
[Remote SPARQL endpoints](#remote-sparql-endpoints)).

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
- `pkg/endpoints/`: the same two modules (`web/` and `node/`), built with every
  feature and the `endpoints` feature (see
  [Remote SPARQL endpoints](#remote-sparql-endpoints))

`import "@rudof/rudof"` and `require("@rudof/rudof")` pick `node/` in Node.js and `web/`
everywhere else (`exports` in `package.json`); `@rudof/rudof/web` and `@rudof/rudof/node`
select one explicitly. `@rudof/rudof/endpoints`, `@rudof/rudof/endpoints/web` and
`@rudof/rudof/endpoints/node` do the same with `endpoints/`.

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

`endpoints` is a further feature, off by default because the `.wasm` it
produces only loads in runtimes with JSPI: see
[Remote SPARQL endpoints](#remote-sparql-endpoints).

`RUDOF_WASM_FEATURES` selects the features that `build.sh` builds `pkg/web/`
and `pkg/node/` with (`pkg/endpoints/` always has every feature):

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
| RDF data | `readData(data, format?, base?, readerMode?, merge?)`, `serializeData(format?)`, `addTriple(subject, predicate, object, strictIris?)`, `removeTriple(subject, predicate, object, strictIris?)`, `addTriples(triples, strictIris?)`, `removeTriples(triples, strictIris?)`, `triples(subject?, predicate?, object?, strictIris?, limit?)`, `dereference(uri, readerMode?, merge?)`, `listEndpoints()` |
| Staging | `beginStaging()`, `commitStaging()`, `rollbackStaging()`, `stagingDepth()` — stage changes to the data, then keep or discard them. Only the data is staged: the schemas stay loaded either way, while a rollback clears the validation results, the query results and the loaded query |
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
generation. `dereference` is there for completeness, but throws a `DataError`:
it needs blocking network access. `listEndpoints` returns `[]` unless the
module is built with the `endpoints` feature (see
[Remote SPARQL endpoints](#remote-sparql-endpoints)). Conversions that render
images or write HTML to a folder fail too.

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

## Remote SPARQL endpoints

With the `endpoints` feature, ShEx and SHACL validation, SPARQL queries and
`nodeInfo` can work on the data of a remote SPARQL endpoint, as with
`rudof --endpoint` on the command line:

```js
const rudof = new Rudof();
rudof.useEndpoint("https://query.wikidata.org/sparql"); // or a known name, e.g. "wikidata"
rudof.readShex(schema);
rudof.readShapemap("<http://www.wikidata.org/entity/Q80>@<Human>");
const report = await rudof.validateShexAsync();

rudof.readQuery("SELECT ?p ?o WHERE { <http://www.wikidata.org/entity/Q80> ?p ?o } LIMIT 10");
const results = await rudof.runQueryAsync();
```

| Method | |
|--------|-|
| `useEndpoint(endpoint, strategy?, options?)` | Uses the endpoint (a URL or the name of a known endpoint) as the data, replacing the current data. `strategy` is `sparql` (the default, SPARQL queries) or `dereference` (fetches each node's IRI as Turtle). `options` overrides the configured request limits: `minRequestIntervalMs`, the minimum milliseconds between two requests (default 1100 for remote hosts, 0 for `localhost`; `0` disables the rate limiter, though a 429 response still makes rudof back off); `maxConcurrentRequests`, the requests in flight at once when fetching the nodes of a shapemap ahead of validation (default 1); and `maxNodesPerQuery`, the nodes fetched per query then (default 20, `1` disables batching). No request is made yet |
| `requestCount()` | The number of HTTP requests sent so far to the endpoint selected with `useEndpoint`, retries included |
| `validateShexAsync()`, `validateShaclAsync(mode?)`, `runQueryAsync()`, `nodeInfoAsync(nodeSelector, predicates?, mode?, showColors?, depth?)` | Like `validateShex`, `validateShacl`, `runQuery` and `nodeInfo`, but return a `Promise`, and can wait for the endpoint |
| `setFetch(fetch)` (a function of the module) | Sets the function used for the HTTP requests, with the signature of `fetch` (`null` restores the global `fetch`) |

The validators are synchronous code, while wasm can only reach the network
through JavaScript's asynchronous `fetch`. The `...Async` methods bridge the two
with [JSPI](https://github.com/WebAssembly/js-promise-integration) (JavaScript
Promise Integration, `#[wasm_bindgen(jspi)]`): while one waits for an HTTP
response, the WebAssembly stack is suspended and the JavaScript event loop
keeps running, so the page stays responsive. In `rudof_rdf` (its `jspi`
feature), the endpoint client is the native one, with its caching, rate
limiting and retries; only the requests (`fetch` instead of `reqwest`), the
timers (`setTimeout`) and the wait for the responses (JSPI instead of a tokio
runtime) differ.

Things to know:

- **Runtimes.** A module built with `endpoints` needs JSPI and exnref, and does
  not load at all without them: Chrome/Edge 137+ and Node.js 25+ have both
  enabled by default, while Firefox and Safari don't ship JSPI yet. That is why
  the feature is not part of the default build: the npm package ships it as a
  separate build, in `endpoints/` (`@rudof/rudof/endpoints`), which `build.sh`
  builds with every other feature too. A module built without it is byte for
  byte the same as before the feature existed.
- **Synchronous methods.** `validateShex`, `runQuery`, etc. keep working on data
  loaded with `readData`. On endpoint data, they throw an error saying to use
  the asynchronous methods when they would need to wait for a response
  (answers already cached by an earlier asynchronous call can be reused).
- **One call at a time per session.** While an `...Async` call is pending, its
  `Rudof` object is in use and its other methods throw; use several `Rudof`
  objects to work concurrently.
- **CORS.** In browsers, the endpoint must allow cross-origin requests (Wikidata
  and most public endpoints do). Otherwise, `setFetch` can route the requests
  through a proxy. Browsers don't let pages set the `User-Agent`; Wikimedia asks
  browser clients to identify themselves with `Api-User-Agent` instead, which
  `setFetch` can add:

  ```js
  setFetch((url, init) =>
    fetch(url, { ...init, headers: { ...init.headers, "Api-User-Agent": "my-app/1.0 (me@example.org)" } }));
  ```
- **Size.** The feature adds about 0.7 MB (4.5%) to the `.wasm` before `wasm-opt`.

## Examples

- Node.js: `node examples/node.cjs`
- Node.js, with a remote endpoint (Wikidata): `node examples/endpoint.cjs`, with
  Node.js 25+ (it uses `pkg/endpoints/`).
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
