# Online demo

The [online demo](../demo/index.html) validates RDF data with ShEx or SHACL in
your browser. It runs `rudof` compiled to WebAssembly, so nothing you type is
sent to a server.

- **ShEx**: enter the RDF data, the ShEx schema and a shape map (for example
  `:alice@:Person`, or `{FOCUS :name _}@:Person` to validate every node with a
  `:name`), and press **Validate**. The result can be shown as a compact or
  detailed table, JSON or CSV.
- **SHACL**: enter the RDF data and the shapes graph, and press **Validate**.
  The validation report can be shown as a table, a one-line summary, or as RDF
  in several formats. The SPARQL engine runs the same validation through
  SPARQL queries.

**Ctrl + Enter** validates from any input. Changing the result format shows
the last result again without validating again.

The demo uses [`@rudof/rudof`](https://www.npmjs.com/package/@rudof/rudof), the
npm package of `rudof`, which can be used in other web pages and in Node.js:

```sh
npm install @rudof/rudof
```

```js
import init, { validateShex, validateShacl } from "@rudof/rudof";

await init(); // in browsers: loads the WebAssembly module
const report = validateShex(data, schema, ":alice@:Person");
console.log(report.conforms, report.entries);
```

Everything is passed as text: the demo can't fetch data, schemas or ShEx
`IMPORT`s from the web.
