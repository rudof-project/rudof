# @rudof/rudof

Validate RDF data with **ShEx** and **SHACL**, and query it with **SPARQL**,
in the browser and in Node.js.

This is the JavaScript package of [rudof](https://github.com/rudof-project/rudof),
compiled to WebAssembly. Its API mirrors rudof's
[Python bindings](https://pypi.org/project/pyrudof/), and it ships TypeScript
declarations.

```sh
npm install @rudof/rudof
```

## Node.js

```js
const { validateShex, validateShacl } = require("@rudof/rudof");
// or: import { validateShex, validateShacl } from "@rudof/rudof";

const data = `
prefix : <http://example.org/>
:alice :name "Alice" ; :age 23 .
:bob   :name "Bob"   ; :age "unknown" .
`;

const schema = `
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string ; :age xsd:integer }
`;

const report = validateShex(data, schema, ":alice@:Person, :bob@:Person");
console.log(report.conforms); // false
for (const e of report.entries) {
  console.log(e.node, e.shape, e.status); // ... bob ... nonconformant
}
```

## Browsers and bundlers

In browsers the WebAssembly module is loaded asynchronously, so call the
default export once before anything else:

```js
import init, { validateShacl } from "@rudof/rudof";

await init();
const report = validateShacl(data, shapes);
```

Bundlers that understand `new URL("...", import.meta.url)` (Vite, webpack 5,
Parcel, Rollup with a URL plugin, esbuild with a file loader) include the
`.wasm` file automatically. Without a bundler, import
`@rudof/rudof/web/rudof_wasm.js` from a CDN or from your own copy of the package, or
give `init` the URL (or the bytes) of the `.wasm` file:

```js
await init({ module_or_path: "/assets/rudof_wasm_bg.wasm" });
```

The file is exported as `@rudof/rudof/rudof_wasm_bg.wasm`.

## Sessions

`validateShex` and `validateShacl` do a whole validation in one call. The
`Rudof` class keeps a session (data, schemas, shapemaps, queries and results)
across calls, as in the Python bindings:

```js
import { Rudof, RudofConfig } from "@rudof/rudof";

const rudof = new Rudof(RudofConfig.fromToml('base_iri = "http://example.org/"'));
rudof.readData(data);                     // Turtle by default
rudof.readShex(schema);                   // ShExC by default
rudof.readShapemap("{FOCUS :name _}@:Person");
const report = rudof.validateShex();      // { conforms, entries, violations }
console.log(rudof.serializeShexValidationResults("compact"));

rudof.readShacl(shapes);
const shaclReport = rudof.validateShacl();

rudof.readQuery("SELECT ?s WHERE { ?s ?p ?o }");
const results = rudof.runQuery();         // { kind: "select", variables, rows }
```

Errors are thrown as `Error`s whose `name` says what failed (`ShExError`,
`DataError`, `QueryError`, ...).

The session also converts and compares schemas, and reads DCTAP profiles,
rdf-config documents, service descriptions and property graph schemas. The
TypeScript declarations list every method, and the
[repository](https://github.com/rudof-project/rudof/tree/master/bindings/wasm)
has a summary of them.

## Remote SPARQL endpoints

`@rudof/rudof/endpoints` is a separate build of the package that can also
validate and query the data of a remote SPARQL endpoint, with asynchronous
methods. It relies on WebAssembly JSPI (JavaScript Promise Integration), so it
only loads in runtimes that support it, e.g. Chrome/Edge 137+ and Node.js 25+
(not Firefox or Safari yet):

```js
import { Rudof } from "@rudof/rudof/endpoints"; // in browsers, `await init()` first, as above

const rudof = new Rudof();
rudof.useEndpoint("https://query.wikidata.org/sparql"); // or a known name, e.g. "wikidata"
rudof.readShex(schema);
rudof.readShapemap("<http://www.wikidata.org/entity/Q80>@<Human>");
const report = await rudof.validateShexAsync();

rudof.readQuery("SELECT ?p ?o WHERE { <http://www.wikidata.org/entity/Q80> ?p ?o } LIMIT 10");
const results = await rudof.runQueryAsync();
```

`validateShaclAsync` and `nodeInfoAsync` work in the same way, and `setFetch`
replaces the function used for the HTTP requests (e.g. to add headers or go
through a proxy). By default rudof waits 1100 ms between two requests to a
remote endpoint (none for `localhost`); for your own endpoint, pass options such
as `rudof.useEndpoint(url, "sparql", { minRequestIntervalMs: 0, maxConcurrentRequests: 4 })`.
`rudof.requestCount()` returns the number of requests sent so far. Everything
else is as in the main build; its `.wasm` file is
exported as `@rudof/rudof/endpoints/rudof_wasm_bg.wasm`. See the
[repository](https://github.com/rudof-project/rudof/tree/master/bindings/wasm#remote-sparql-endpoints)
for the details.

## Limitations

Everything is passed as strings: reading files, fetching or dereferencing
IRIs, ShEx `IMPORT`s, rendering images and property graph databases are not
available, and remote SPARQL endpoints only in `@rudof/rudof/endpoints` (see
above). Fetch remote documents yourself (e.g. with
`fetch`) and pass their text in. SPARQL over the loaded data (queries,
SHACL-SPARQL constraints, ShapeMap query selectors) runs on an embedded
Oxigraph store.

## License

MIT or Apache-2.0, at your option.
