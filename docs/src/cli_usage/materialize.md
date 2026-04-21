# materialize

The `materialize` command generates an RDF graph from a ShEx schema that contains [Map semantic actions](https://shex.io/shex-semantics/index.html#semantic-actions) and a MapState file that binds the mapped variables to RDF nodes.

A MapState is a JSON file that maps each variable IRI declared in a Map semantic action to an RDF node (IRI or literal).
It is typically produced as the output of ShEx validation with Map extensions, and then passed to `materialize` to construct the corresponding RDF graph.

For the examples below we assume the following files are available in the `examples/shex` folder.

`person_map.shexj` — a ShEx schema with Map semantic actions:

```json
{
  "@context": "http://www.w3.org/ns/shex.jsonld",
  "type": "Schema",
  "shapes": [{
    "type": "ShapeDecl",
    "id": "http://example.org/PersonShape",
    "shapeExpr": {
      "type": "Shape",
      "expression": {
        "type": "EachOf",
        "expressions": [
          {
            "type": "TripleConstraint",
            "predicate": "http://example.org/name",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/name>"
            }]
          },
          {
            "type": "TripleConstraint",
            "predicate": "http://example.org/email",
            "semActs": [{
              "type": "SemAct",
              "name": "http://shex.io/extensions/Map/",
              "code": "<http://example.org/email>"
            }]
          }
        ]
      }
    }
  }]
}
```

`person_map_state.json` — a MapState file binding the mapped variables to RDF nodes:

```json
{
  "http://example.org/name":  {"Iri": "http://example.org/Alice"},
  "http://example.org/email": {"Iri": "mailto:alice@example.org"}
}
```

Both files are located in the [examples/shex folder](https://github.com/rudof-project/rudof/tree/master/examples/shex) and can be downloaded with:

```sh
curl -o person_map.shexj https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/shex/person_map.shexj
curl -o person_map_state.json https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/shex/person_map_state.json
```

## Basic materialization

Materialize an RDF graph using a ShExJ schema and a MapState file:

```sh
$ rudof materialize -s examples/shex/person_map.shexj -f shexj -m examples/shex/person_map_state.json
_:1 <http://example.org/name> <http://example.org/Alice> ;
	<http://example.org/email> <mailto:alice@example.org> .
```

By default the root subject node is a fresh blank node. Use `--node` to provide a named IRI instead:

```sh
$ rudof materialize -s examples/shex/person_map.shexj -f shexj -m examples/shex/person_map_state.json -n "http://example.org/alice"
<http://example.org/alice> <http://example.org/name> <http://example.org/Alice> ;
	<http://example.org/email> <mailto:alice@example.org> .
```

## Choosing the output format

The `--result-format` (`-r`) flag controls the serialization of the generated graph.
Supported values include `turtle`, `ntriples`, `rdfxml`, `trig`, `n3`, and `nquads`.

```sh
$ rudof materialize -s examples/shex/person_map.shexj -f shexj -m examples/shex/person_map_state.json -r ntriples
_:1 <http://example.org/name> <http://example.org/Alice> .
_:1 <http://example.org/email> <mailto:alice@example.org> .
```

## Writing the result to a file

Use `--output-file` (`-o`) to write the materialized graph to a file instead of the terminal:

```sh
rudof materialize -s examples/shex/person_map.shexj -f shexj -m examples/shex/person_map_state.json -o result.ttl
```

## Usage

```sh
Materialize an RDF graph from a ShEx schema and Map semantic-action state

Usage: rudof materialize [OPTIONS] --schema <INPUT>

Options:
  -s, --schema <INPUT>          ShEx schema, FILE, URI or - for stdin
  -f, --schema-format <FORMAT>  ShEx schema format (ShExC, ShExJ, ...), default = ShExC [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads]
      --reader-mode <MODE>      RDF reader mode (strict or lax) [default: strict] [possible values: lax, strict]
  -b, --base <IRI>              Base IRI for the schema
  -m, --map-state <FILE>        JSON file containing the MapState produced by ShEx validation with Map semantic actions
  -n, --node <IRI>              IRI of the root subject node; a fresh blank node is used when omitted
  -r, --result-format <FORMAT>  RDF output format for the materialized graph (Turtle, NTriples, ...) [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, compact, json, plantuml, svg, png]
  -c, --config-file <FILE>      Config file name
  -o, --output-file <FILE>      Output file name, default = terminal
      --force-overwrite         Force overwrite to output file if it already exists
  -h, --help                    Print help
```
