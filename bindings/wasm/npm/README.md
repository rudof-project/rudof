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

## Limitations

Everything is passed as strings: reading files, fetching or dereferencing
IRIs, ShEx `IMPORT`s, remote SPARQL endpoints, rendering images and property
graph databases are not available. Fetch remote documents yourself (e.g. with
`fetch`) and pass their text in. SPARQL over the loaded data (queries,
SHACL-SPARQL constraints, ShapeMap query selectors) runs on an embedded
Oxigraph store.

## License

MIT or Apache-2.0, at your option.
