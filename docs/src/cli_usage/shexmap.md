# shexmap

The `shexmap` command maps RDF from one ShEx schema to another with
[ShExMap](http://shex.io/extensions/Map/) semantic actions.  A `%Map:{ ... %}` action on a
triple constraint names a variable: in the **input schema** it binds the variable to the value
the constraint matched; in the **output schema** it says where the bound value goes.  Two
functions transform values on the way -- `regex(/(?<v:family>[a-zA-Z]+), (?<v:given>[a-zA-Z]+)/)`
splits and joins, `hashmap(v:status, {"D": "Divorced", "M": "Married"})` recodes -- and
`id(v:mrn)` on a shape-valued constraint names the node the output builds, so that two
constraints with the same key share it.

The command **binds** an input node against the input schema, then **materializes** the output
schema from the bindings.  Either half runs alone: `--bindings-out` writes the bindings as JSON,
`--bindings` reads them back (the JSON is shared with [shex.js's
extension-map](https://github.com/shexjs/shex.js/tree/main/packages/extension-map) and
[PyShEx](https://github.com/linkml/PyShEx)), and `--check` analyses a schema pair without data.

The examples below use the files in
[`examples/shexmap/shexjs`](https://github.com/rudof-project/rudof/tree/master/examples/shexmap/shexjs),
whose relative IRIs resolve against `http://a.example/schema/`; run them from that directory.

## Map a graph

`BPfhir-instance.ttl` holds a blood pressure reading in a FHIR-like layout; `BPfhir-schema.shex`
binds its parts (the patient's `bp:given` and `bp:family` names, the systolic and diastolic
`bp:sysVal`, `bp:sysUnits`, `bp:diaVal`, `bp:diaUnits`), and `BPdam-schema.shex` places them in a
flat layout, joining the names with `regex()`:

```sh
$ rudof shexmap BPfhir-instance.ttl -s BPfhir-schema.shex -n '<tag:BPfhir123>' \
    -O BPdam-schema.shex --root '<tag:b0>' --output-shape BPunitsDAM \
    --base-schema http://a.example/schema/
@prefix bp: <http://shex.io/extensions/Map/#BPDAM-> .
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix Map: <http://shex.io/extensions/Map/#> .
@prefix : <http://dam.example/med#> .
<tag:b0> :systolic _:b0 ;
	:diastolic _:b1 ;
	:name "Walker, Alice" .
_:b0 :value "110"^^xsd:float ;
	:units "mmHg" .
_:b1 :value "70"^^xsd:float ;
	:units "mmHg" .
```

* `-s`/`--schema` is the input schema and `-n`/`--node` the node to map; `-l`/`--shape-label`
  names the input shape (default: the schema's start).
* `-O`/`--output-schema` is the output schema; `--root` the node to build (default: a blank
  node) and `--output-shape` the shape to build (default: the output schema's start).
* `-r`/`--result-format` chooses the serialization and `-o`/`--output-file` the destination,
  as for the other commands.

Before binding, the input node is validated against the input schema, so a node that does not
conform fails with the validator's diagnostics (`--no-validate` skips this).

## Repeated and nested constraints

When the input schema repeats a constraint (`fhir:item @<BPfhir>*`, `:contact { ... }*`), each
match makes one *iteration* of bindings, and the output repetition that reads those variables
runs once per iteration, so everything bound from one reading stays together.  A binding made
above the repetition (a patient's name beside the list of readings) is read in every iteration.
Blank nodes the output builds are determined by the root, the constraint and the input node the
iteration matched, so materializing the same input twice gives the same graph.

## Bindings

`-b`/`--bindings-out` writes the bindings as JSON (`-` for the terminal):

```sh
$ rudof shexmap BPfhir-instance.ttl -s BPfhir-schema.shex -n '<tag:BPfhir123>' -b -
{
  "http://shex.io/extensions/Map/#BPDAM-diaUnits": {
    "value": "mmHg"
  },
  "http://shex.io/extensions/Map/#BPDAM-diaVal": {
    "type": "http://www.w3.org/2001/XMLSchema#float",
    "value": "70"
  },
  "http://shex.io/extensions/Map/#BPDAM-family": {
    "value": "Walker"
  },
  ...
}
```

A scope whose shape has repeated constraints is written as an array: its own bindings first,
then one list per repeated constraint or group, each list holding one scope per iteration; the
iterations of a repeated shape-valued constraint also record the input node they matched under
`"@node"`.  `-j`/`--bindings` materializes from such a file instead of binding data:

```sh
$ rudof shexmap -j BP-simple-bindings.json -O BPdam-schema.shex --root '<tag:b0>' \
    --output-shape BPunitsDAM --base-schema http://a.example/schema/ -r ntriples
```

`--static FILE` supplies extra variable values available everywhere, as a JSON object of
variable IRIs to terms (`"\"123-456\""`, `"<http://...>"` or `{"value": "...", "type": "..."}`).

## Ambiguity

An input may conform to its schema in several ways that bind differently (two components that
both fit a systolic and a diastolic constraint); the output schema may accept the bindings in
several ways (a `OneOf` whose alternatives are both bound).  The command warns and uses the
first parse and the materialization that reads the most bindings; `--strict` makes either an
error:

```sh
$ rudof shexmap bp-reading.ttl -s bp-ambiguous-schema.shex -n '<reading1>' -O bp-dam-schema.shex \
    --root '<tag:bp1>' --base-schema http://a.example/schema/ --base-data http://a.example/turtle/ --strict
Error: ShExMap error: <http://a.example/turtle/reading1> matches the input schema in 2 ways that bind different values
```

## Check a schema pair

`--check` analyses the two schemas without data: every variable the output reads must be bound
by the input where it can be read from (once per iteration of the same repetition, or above it),
every output repetition must have one input list to iterate, and `id()` must sit on a
shape-valued constraint.  Bound variables the output never reads are warnings.  The exit code is
1 when there are errors.

```sh
$ rudof shexmap --check -s BPfhir-schema.shex -O card-flat-schema.shex --base-schema http://a.example/schema/
error: card:fullName reads :name, which the input schema never binds
error: card:phone reads :tel, which the input schema never binds
error: card:mbox reads :email, which the input schema never binds
warning: bp:diaUnits is bound (at the root) but the output never reads it
...
Error: the schemas do not map coherently
```

Static variables named in `--static` count as bound.

## Update a graph in place

`--into FILE` materializes into an existing graph: what the output schema currently holds at
the root is replaced (the schema, read as an input schema on that graph, says which triples it
governs), everything else is kept, and the graph is written back to the file unless
`-o` names another one.  The graph must conform to the output schema at the root, else there is
no telling what to replace.

```sh
$ rudof shexmap BPfhir-instance.ttl -s BPfhir-schema.shex -n '<tag:BPfhir123>' \
    -O BPdam-schema.shex --root '<tag:b0>' --output-shape BPunitsDAM \
    --base-schema http://a.example/schema/ --into dam.ttl
shexmap: dam.ttl: 7 triples added, 0 removed
```

## Provenance

`--provenance FILE` writes one JSON object per output triple: the triple, the output constraint
it came from, the input scope (the list and iteration indices) and node it was evaluated at, the
bindings read for it, and how the object arose (`variable`, `code`, `constant`, `structural`,
`named` or `keyed`).

## Relation to `materialize`

[`materialize`](./materialize.md) builds a graph from the flat `MapState` the Map semantic
actions record during validation, one value per variable.  `shexmap` reads the input graph
again after validation and keeps the structure of what each constraint matched, so it handles
repeated and nested constraints, functions, keys and in-place updates; it does not use the
`MapState`.

## Usage

```sh
Map RDF between two ShEx schemas with ShExMap (%Map:{ %}) actions: bind, then materialize

Usage: rudof shexmap [OPTIONS] [DATA]...

Arguments:
  [DATA]...  Input RDF data: FILE, URI or - for stdin

Options:
  -t, --data-format <FORMAT>           RDF data format [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
  -s, --schema <INPUT>                 Input ShEx schema (with %Map:{ %} actions that bind): FILE, URI or - for stdin
  -f, --schema-format <FORMAT>         Input schema format (ShExC, ShExJ, ...) [default: shexc]
  -n, --node <NODE>                    Input node to map: <iri>, iri or _:label
  -l, --shape-label <LABEL>            Input shape label (default: the input schema's start)
  -j, --bindings <FILE>                Read bindings JSON (as written by --bindings-out, shex.js or PyShEx) instead of binding data
  -b, --bindings-out <FILE>            Write the bindings as JSON here ('-' for the output)
  -O, --output-schema <INPUT>          Output ShEx schema (with %Map:{ %} actions that place the bindings): FILE, URI or - for stdin
      --output-schema-format <FORMAT>  Output schema format (ShExC, ShExJ, ...) [default: shexc]
      --root <NODE>                    Output node to build: <iri>, iri or _:label (default: a blank node)
      --output-shape <LABEL>           Output shape label (default: the output schema's start)
      --static <FILE>                  JSON object of extra variable values ("<iri>": "\"literal\"" or a {"value": ...} term), as shex.js's staticVars
      --strict                         Fail when the input or the output can be matched in more than one way
      --no-validate                    Skip the ShEx validator's check of the input node before binding
      --into <FILE>                    An existing RDF file (in --result-format) to update in place: what the output schema currently holds at --root is replaced, the rest kept; written back there unless --output-file says otherwise
      --provenance <FILE>              Write one JSON object per output triple here: the triple, the output constraint's predicate, the input scope and node it came from, the bindings read, and how the object arose
      --check                          Only analyse --schema against --output-schema, without data: every output repetition has a list to iterate, every variable read is bound where it can be read; exit 1 on errors
      --base-schema <IRI>              Base IRI for relative IRIs in the schemas and shape labels
      --base-data <IRI>                Base IRI for relative IRIs in the RDF data and nodes
      --reader-mode <MODE>             RDF reader mode (strict or lax) [default: strict] [possible values: lax, strict]
  -r, --result-format <FORMAT>         RDF output format for the materialized graph (Turtle, NTriples, ...) [default: turtle]
  -c, --config-file <FILE>             Config file name
  -o, --output-file <FILE>             Output file name, default = terminal
      --force-overwrite                Force overwrite to output file if it already exists
  -h, --help                           Print help
```
