# shex-validate

This command can be used to validate RDF data using ShEx. This is a specific command for ShEx.

## Examples

### Validate data using a ShEx schema and ShapeMap

```sh
$ rudof shex-validate -s examples/user.shex -m examples/user.sm examples/user.ttl

╭──────┬───────┬────────┬──────────────────────────────────────────────────────────────────────────────────╮
│ Node │ Shape │ Status │ Details                                                                          │
├──────┼───────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :a   │ :User │ OK     │ Shape passed :a@:User                                                            │
├──────┼───────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :b   │ :User │ OK     │ Shape passed :b@:User                                                            │
├──────┼───────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :c   │ :User │ OK     │ Shape passed :c@:User                                                            │
├──────┼───────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :d   │ :User │ FAIL   │ Datatype error on node :d for property schema:name: found xsd:integer, expected  │
│      │       │        │ xsd:string, lexical form "23"^^xsd:integer                                       │
├──────┼───────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :e   │ :User │ FAIL   │ Shape :User failed for node :e                                                   │
│      │       │        │ └── References failed:                                                           │
│      │       │        │     └── Predicate schema:knows -> :d as :User                                    │
│      │       │        │         └── Shape :User failed for node :d: no candidates matched the expression │
│      │       │        │             └── Condition failed for predicate schema:name on node "23"^^<http:/ │
│      │       │        │ /www.w3.org/2001/XMLSchema#integer>: Datatype error: found <http://www.w3.org/20 │
│      │       │        │ 01/XMLSchema#integer>, expected <http://www.w3.org/2001/XMLSchema#string>, lexic │
│      │       │        │ al form "23"^^<http://www.w3.org/2001/XMLSchema#integer>                         │
├──────┼───────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :f   │ :User │ FAIL   │ No candidates to match. Missing: [schema:name]                                   │
╰──────┴───────┴────────┴──────────────────────────────────────────────────────────────────────────────────╯
```

### Validate data using ShEx schema, a node and a shape

```sh
$ rudof shex-validate -s examples/user.shex -n "http://example.org/a" -l "http://example.org/User" examples/user.ttl

╭──────┬───────┬────────┬───────────────────────╮
│ Node │ Shape │ Status │ Details               │
├──────┼───────┼────────┼───────────────────────┤
│ :a   │ :User │ OK     │ Shape passed :a@:User │
╰──────┴───────┴────────┴───────────────────────╯
```

### Show the triples that the validation reads

Use `--statistics` to show these numbers:

- The number of triples that the validation reads. This number includes duplicates.
- The number of distinct triples.
- The number of valid triples and the number of invalid triples. Refer to [Extract the valid and invalid subgraphs](#extract-the-valid-and-invalid-subgraphs).

The numbers include only the arcs that the validator gets from the data.

`--statistics` counts the valid and invalid triples, but it does not keep them. To keep them, use `--valid` or `--invalid`.

Use `--visited <FILE>` to write the triples that the validation reads to a file. The default format is `turtle`. To change the format, use `--visited-format`.

```sh
$ rudof shex-validate -s examples/user.shex -n "http://example.org/a" -l "http://example.org/User" examples/user.ttl --statistics --visited a.ttl

╭──────┬───────┬────────┬───────────────────────╮
│ Node │ Shape │ Status │ Details               │
├──────┼───────┼────────┼───────────────────────┤
│ :a   │ :User │ OK     │ Shape passed :a@:User │
╰──────┴───────┴────────┴───────────────────────╯
Triples read: 12 (distinct: 9)
Valid triples: 9
Invalid triples: 0
```

> **Note:** `--validated-graph` and `--validated-graph-format` are the old names of `--visited` and `--visited-format`. You can continue to use the old names.

### Extract the valid and invalid subgraphs

The triples that the validation reads can include triples of nodes that do not conform. Use these options to write only a part of the triples:

- `--valid <FILE>` writes the valid subgraph to a file.
- `--invalid <FILE>` writes the invalid subgraph to a file.

The valid subgraph contains these triples:

- The triples that the triple constraints of a conformant `(node, shape)` pair match.
- The valid triples of each node that a conformant pair refers to with a shape reference. This rule applies again to the referenced pairs.

The valid subgraph does not contain these triples:

- The triples of a pair that does not conform.
- The values of an `EXTRA` predicate that no triple constraint matches.
- The triples that the validator examines for a `NOT` expression.

The invalid subgraph contains the triples that the validation reads and that are not in the valid subgraph. Thus, each triple that the validation reads is in one of the two subgraphs. If a conformant pair matches a triple, the triple is valid. This is also true when the validator reads the same triple for a pair that does not conform.

Each output has a format option: `--visited-format`, `--valid-format` and `--invalid-format`. The default format is `turtle`. A Turtle output declares the prefixes of the data. You can write the three outputs with one command.

#### Example

This is the schema:

```shex
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
:Person { :name xsd:string }
```

This is the data:

```turtle
prefix : <http://example.org/>
:alice :name "Alice" ; :email "alice@example.org" .
:bob :name "Robert", "Bob" ; :email "bob@example.org" .
```

The shape map `map.sm` contains `:alice@:Person, :bob@:Person`. Bob has two names. Thus, Bob does not conform to `:Person`.

```sh
$ rudof shex-validate data.ttl -s schema.shex -m map.sm --valid valid.ttl --invalid invalid.nt --invalid-format ntriples
```

The file `valid.ttl` contains only the name of Alice:

```turtle
@prefix : <http://example.org/> .
:alice :name "Alice" .
```

The file `invalid.nt` contains the two names of Bob:

```text
<http://example.org/bob> <http://example.org/name> "Bob" .
<http://example.org/bob> <http://example.org/name> "Robert" .
```

These options are also available in the [shell](shell.md). This command uses the data and the schema that the shell has loaded:

```sh
rudof> shex-validate -m map.sm --statistics --valid valid.ttl --invalid invalid.ttl
```

## Precompiled schemas

For workloads that validate against the same schema many times the AST to IR compilation step can be
done once and cached to a file. See the [precompiled ShEx schemas how-to](../using-rudof/precompiled-shex-schemas.md)
for the full workflow.

### Loading a precompiled cache

`--compiled-schema <FILE>` replaces `--schema`. Conflicts with `--schema`, `--schema-format`,
`--base-schema`, and `--external-resolver`.

```sh
rudof shex-validate \
  --compiled-schema user.ircache \
  --shapemap examples/user.sm \
  examples/user.ttl
```

Equivalently, `binary` is a regular value of `-f`/`--schema-format`, so
`-s`/`--schema` also loads a cache when paired with it -- no dedicated flag
needed:

```sh
rudof shex-validate \
  --schema user.ircache --schema-format binary \
  --shapemap examples/user.sm \
  examples/user.ttl
```

### Compiling as a side-effect

`--compile-to <FILE>` can be passed alongside `--schema`. Subsequent runs can then drop `--schema` and `--compile-to` and use
`--compiled-schema` instead.

```sh
rudof shex-validate \
  --schema examples/user.shex \
  --compile-to user.ircache \
  --shapemap examples/user.sm \
  examples/user.ttl
```

## External-shape resolvers

A ShEx schema may declare a shape as `EXTERNAL`, meaning the definition lives outside the schema and is resolved by an implementation-defined mechanism. By default `rudof` rejects every `EXTERNAL` shape via the built-in `reject-all` resolver, so validation against an unsubstituted external shape always fails.

Without any resolver, `:alice` already fails because `:Address` is rejected outright:

```sh
$ rudof shex-validate -s examples/person.shex -n "http://example.org/alice" -l "http://example.org/Person" examples/person.ttl

╭────────┬─────────┬────────┬──────────────────────────────────────────────────────────────────────────────────╮
│ Node   │ Shape   │ Status │ Details                                                                          │
├────────┼─────────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :alice │ :Person │ FAIL   │ Shape :Person failed for node :alice                                             │
│        │         │        │ └── References failed:                                                           │
│        │         │        │     └── Predicate :address -> :alice_addr as :Address                            │
│        │         │        │         └── External shape :Address rejected for node: :alice_addr and resolver  │
│        │         │        │ reject-all: EXTERNAL shape rejected: no resolver supplied a definition           │
╰────────┴─────────┴────────┴──────────────────────────────────────────────────────────────────────────────────╯
```

Use `--external-resolver` (repeatable) to install resolvers that substitute or judge `EXTERNAL` shapes. Resolvers are applied in the order they appear on the command line (the most recently registered is consulted first, and the default `reject-all` always sits at the tail of the chain).

### Listing the available resolver kinds

```sh
$ rudof shex-validate --list-external-resolvers

Available external-shape resolvers:

  reject-all         Reject any EXTERNAL shape that no earlier resolver claimed
  schema:<path>      Substitute EXTERNAL shape declarations using definitions from a ShEx file
```

When `--list-external-resolvers` is set, the other arguments (including `--schema`) are not required.

### `schema:<path>`

The `schema` resolver loads a separate ShEx file and, during AST→IR compilation, replaces any `EXTERNAL` shape declaration in the main schema with the matching definition from the file.

With the externs file plugged in, `:alice` validates and `:bob` still fails because his address is missing `:city`:

```sh
$ rudof shex-validate -s examples/person.shex --external-resolver schema:examples/address.shex -n "http://example.org/alice" -l "http://example.org/Person" examples/person.ttl

╭────────┬─────────┬────────┬─────────────────────────────╮
│ Node   │ Shape   │ Status │ Details                     │
├────────┼─────────┼────────┼─────────────────────────────┤
│ :alice │ :Person │ OK     │ Shape passed :alice@:Person │
╰────────┴─────────┴────────┴─────────────────────────────╯

$ rudof shex-validate -s examples/person.shex --external-resolver schema:examples/address.shex -n "http://example.org/bob" -l "http://example.org/Person" examples/person.ttl

╭──────┬─────────┬────────┬──────────────────────────────────────────────────────────────────────────────────╮
│ Node │ Shape   │ Status │ Details                                                                          │
├──────┼─────────┼────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ :bob │ :Person │ FAIL   │ Shape :Person failed for node :bob                                               │
│      │         │        │ └── References failed:                                                           │
│      │         │        │     └── Predicate :address -> :bob_addr as :Address                              │
│      │         │        │         └── Shape :Address failed for node :bob_addr: no candidates matched the  │
│      │         │        │ expression                                                                       │
│      │         │        │             └── Candidate [:street "Oak Avenue"] rejected: predicate :city requi │
│      │         │        │ red cardinality {1, 1} but got 0                                                 │
╰──────┴─────────┴────────┴──────────────────────────────────────────────────────────────────────────────────╯
```

### Invalid specs

A malformed spec is reported up-front with a hint listing the recognised kinds:

```sh
$ rudof shex-validate -s examples/person.shex --external-resolver bogus examples/person.ttl

Error: ShEx error: Invalid external-shape resolver spec 'bogus': Unknown external resolver kind 'bogus'. Available kinds: reject-all, schema
```

## IRI normalization modes

The `--node` and `--shape-label` values are parsed as ShapeMap selectors, which normally require IRIs to be enclosed in angle brackets (`<http://example.org/Alice>`). `rudof` supports two modes to control how plain strings are handled.

### Lax mode (default)

In lax mode any string that contains `://` and is not already wrapped in `<>` is automatically wrapped before parsing:

```sh
# Equivalent in lax mode:
rudof shex-validate -s schema.shex -n "http://example.org/a" -l "http://example.org/User" data.ttl
rudof shex-validate -s schema.shex -n "<http://example.org/a>" -l "<http://example.org/User>" data.ttl
```

**Limitations.** The `://` heuristic will fail for IRIs that do not contain `://` (URNs, `mailto:`, `data:` URIs) and may mis-classify a prefixed local name that happens to contain `://`. See [IRI normalization internals](../internals/iri-normalization.md) for details and planned improvements.

### Strict mode

Pass `--strict-iris` to require angle brackets on every IRI. Bare IRIs produce a parse error immediately.

```sh
rudof shex-validate -s schema.shex \
  -n "<http://example.org/a>" \
  -l "<http://example.org/User>" \
  data.ttl --strict-iris
```

Use strict mode in automated pipelines or when the data contains non-`http` IRI schemes.

## Usage

```sh
Validate RDF using ShEx schemas

Usage: rudof shex-validate [OPTIONS] [DATA]...

Arguments:
  [DATA]...

Options:
  -s, --schema <INPUT>            Schema file name, URI or - (for stdin). If omitted, reuses the currently loaded schema
      --compiled-schema <FILE>    Precompiled ShEx SchemaIR cache file.
      --compile-to <FILE>         Compile the ShEx schema and write the precompiled SchemaIR cache to FILE.
  -f, --schema-format <FORMAT>    ShEx Schema format [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads, plantuml, svg, png, binary]
  -m, --shapemap <INPUT>          ShapeMap
      --shapemap-format <FORMAT>  ShapeMap format [possible values: compact, internal, json, details, csv]
  -n, --node <NODE>               Node to validate
      --sort_by <SORT_MODE>       Sort result by (default = node) [default: node] [possible values: node, shape, status, details]
  -l, --shape-label <LABEL>       shape label (default = START)
  -t, --data-format <FORMAT>      RDF Data format [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
      --base-schema <IRI>         Base Schema (used to resolve relative IRIs in Schema)
      --base-data <IRI>           Base RDF Data IRI (used to resolve relative IRIs in RDF data)
      --reader-mode <MODE>        RDF Reader mode [default: strict] [possible values: lax, strict]
  -r, --result-format <FORMAT>    Ouput result format [default: details] [possible values: details, turtle, ntriples, rdfxml, trig, n3, nquads, compact, json, csv]
      --map-state <FILE>          MapState file name
      --strict-iris               Require <> brackets around IRIs (strict mode). By default bare http://… IRIs are accepted (lax mode).
      --strategy <STRATEGY>       How -e/--endpoint answers lookups [default: sparql] [possible values: sparql, dereference]
      --external-resolver <SPEC>  External-shape resolver spec. Repeatable. Syntax: <kind>[:<arg>]. Built-in kinds: 'reject-all', 'schema:<path>'. Use --list-external-resolvers to enumerate.
      --list-external-resolvers   Print the available external-shape resolver kinds and exit
      --show-intermediate-results Print each (node, shape) result as soon as it's computed, instead of only the final report
      --statistics                Show the number of triples that the validation reads, the number of distinct triples, and the number of valid and invalid triples
      --visited <FILE>            Write all the triples that the validation reads to FILE
      --visited-format <FORMAT>   RDF format of the --visited file [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
      --valid <FILE>              Write the valid subgraph to FILE: the triples that the conformant node/shape pairs match
      --valid-format <FORMAT>     RDF format of the --valid file [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
      --invalid <FILE>            Write the invalid subgraph to FILE: the triples that the validation reads and that are not valid
      --invalid-format <FORMAT>   RDF format of the --invalid file [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
  -c, --config-file <FILE>        Config file name
  -o, --output-file <FILE>        Output file name, default = terminal
      --force-overwrite           Force overwrite to output file if it already exists
  -h, --help                      Print help
```

## Selecting the RDF backend

By default, validation data is loaded into an in-process `memory` graph. Use `--backend` to switch to a QLever Docker container or a remote SPARQL endpoint:

```sh
rudof shex-validate -s schema.shex -m shapemap.sm --backend qlever data.ttl
rudof shex-validate -s schema.shex -m shapemap.sm --endpoint https://my.sparql.server/sparql
```

See the [RDF backend (`--backend`) reference](./backend.md) for full documentation. For a Wikibase endpoint (Wikidata, MaRDI, …) `--strategy dereference` is an alternative to SPARQL that fetches each node's own IRI directly over HTTP instead — see the "Alternative endpoint access" section on that page.
