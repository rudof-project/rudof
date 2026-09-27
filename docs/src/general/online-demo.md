# Online demo

The [online demo](../demo/index.html) converts RDF data and property graphs
between formats, validates them with ShEx, SHACL or PGSchema, and queries RDF
data with SPARQL, in your browser. It runs `rudof` compiled to WebAssembly, so
nothing you type is sent to a server. It has three sections, each with its own
tabs:

**Data**

- **RDF**: enter RDF data in any format, choose the output format (Turtle,
  N-Triples, RDF/XML, JSON-LD, TriG, N3, N-Quads, or the source of a PlantUML
  diagram) and press **Convert**. **Copy** copies the result, and
  **Use as input** replaces the input with it, to convert it again.
- **Property graph**: enter a property graph in
  [YARS-PG](https://github.com/lszeremeta/yarspg) syntax and press **Convert**
  to check it and see it as YARS-PG (sorted by name) or as JSON.

**Validate**

- **ShEx**: enter the RDF data, the ShEx schema and a shape map (for example
  `:alice@:Person`, or `{FOCUS :name _}@:Person` to validate every node with a
  `:name`), and press **Validate**. The result can be shown as a compact or
  detailed table, JSON or CSV.
- **SHACL**: enter the RDF data and the shapes graph, and press **Validate**.
  The validation report can be shown as a table, a one-line summary, or as RDF
  in several formats. The SPARQL engine runs the same validation through
  SPARQL queries.
- **PGSchema**: enter a property graph (in YARS-PG), a PGSchema and a type map,
  which says which type each node or edge should have (for example
  `n1: PersonType, e1: KnowsType`), and press **Validate**. The result is a
  table with each node or edge, whether it conforms and why, or the compact,
  JSON or CSV output of `rudof`.

**Query**

- **SPARQL**: enter RDF data and a SPARQL query (**Examples** has one of each
  kind), and press **Run query**. `SELECT` results are shown as a table (or JSON
  or CSV), `ASK` as true or false, and the graph of `CONSTRUCT` and `DESCRIBE`
  in any RDF format.

**Ctrl + Enter** runs the tab from any of its inputs. Changing the result
format shows the last result again without running it again. Links can point
to a tab, for example `demo/index.html#validate/shacl` or `#query/sparql`.

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
