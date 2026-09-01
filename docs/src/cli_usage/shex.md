# ShEx

The command `shex` can be used to obtain information about [ShEx](https://shex.io/) schemas.

> For ShEx validation, you can use the `shex-validate` or `validate` commands

For executing the examples in this page we assume you have a file called `user.shex` in your folder with the following contents:

```shexc
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>
prefix schema: <http://schema.org/>

start = @:User

:User {
  schema:name   xsd:string             ;
  schema:knows  @:User               * ;
  :status       [ :Active :Waiting ] ? ;
}
```

It is located in the [examples folder](https://github.com/rudof-project/rudof/tree/master/examples) and can directly be downloaded running the following commands:

```sh
curl -o user.shex https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/user.shex
```

## Information about the ShEx schema

You can obtain information about a ShEx schema using the following command:

```sh
rudof shex -s examples/user.shex
```

## Checking if the schema is well formed

In ShEx, there are some [requirements](https://shex.io/shex-semantics/index.html#schema-requirements) that the schemas have to meet before validating. For example, for schemas that have recursive shapes and negations, it is required that there are no cycles with negative references, i.e. the schemas should have stratified negation. An example of a non-stratified schema could be:

```shex
prefix :       <http://example.org/>

:S {
    :p NOT @:S +
}
```

If you try to check that schema with rudof, it informs about the error:

```sh
$ rudof shex -s examples/shex/non_stratified.shex
Error: ShEx error: Failed to compile ShEx schema: Schema contains negative cycles in its dependency graph. Found 1 negative cycle(s).
```

## Obtaining information about a shape

Sometimes, it can be useful to obtain information about a specific shape in a schema:

```
$ rudof shex -s examples/shex/figures.shex -l ":ColouredFigure"
```

## Conversion between ShEx formats

It is possible to use `rudof` to convert between different ShEx formats as:

```sh
❯ rudof shex -s examples/user.shex -r shexj
```

the output will be:

```json
{
  "@context": "http://www.w3.org/ns/shex.jsonld",
  "type": "Schema",
  "start": "http://example.org/User",
  "shapes": [
    {
      "type": "ShapeDecl",
      "id": "http://example.org/User",
      "abstract": false,
      "shapeExpr": {
        "type": "Shape",
        "expression": {
          "type": "EachOf",
          "expressions": [
            {
              "type": "TripleConstraint",
              "predicate": "http://schema.org/name",
              "valueExpr": {
                "type": "NodeConstraint",
                "datatype": "http://www.w3.org/2001/XMLSchema#string"
              }
            },
            {
              "type": "TripleConstraint",
              "predicate": "http://schema.org/knows",
              "valueExpr": "http://example.org/User",
              "min": 0,
              "max": -1
            },
            {
              "type": "TripleConstraint",
              "predicate": "http://example.org/status",
              "valueExpr": {
                "type": "NodeConstraint",
                "values": [
                  "http://example.org/Active",
                  "http://example.org/Waiting"
                ]
              },
              "min": 0,
              "max": 1
            }
          ]
        }
      }
    }
  ],
  "prefixmap": {
    "": "http://example.org/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "schema": "http://schema.org/"
  }
}
```

A ShEx schema can also be serialized as RDF, using the [ShExR](http://shex.io/shex-semantics/#shexr) vocabulary — pass `-r turtle`, `ntriples`, `rdfxml`, `trig`, `n3` or `nquads`:

```sh
❯ rudof shex -s examples/person.shex -r turtle
```

```turtle
@prefix xsd: <http://www.w3.org/2001/XMLSchema#> .
@prefix sx: <http://www.w3.org/ns/shex#> .
@prefix : <http://example.org/> .
:Address a sx:ShapeDecl ;
	sx:shapeExpr _:2 .
:Person a sx:ShapeDecl ;
	sx:shapeExpr _:4 .
_:1 a sx:Schema ;
	sx:shapes _:b .
...
```

The `sx:` prefix (for the ShExR vocabulary itself) is always added, and the schema's own prefixes — here `:` and `xsd:`, from `examples/person.shex`'s own `PREFIX` declarations — are carried over too.

This works the same way in the shell — `shex examples/person.shex` to load the schema, then `shex -r turtle` to show it as RDF.

## Loading a ShEx schema from RDF (ShExR)

The reverse direction also works: `-s`/`--schema` accepts a ShExR-encoded RDF file as input, not just ShExC/ShExJ — pass `-f turtle`, `ntriples`, `rdfxml` or `nquads` (`trig` and `n3` aren't supported as *input* formats yet, only as output):

```sh
❯ rudof shex -s examples/person.shexr.ttl -f turtle -r shexc
```

This round-trips with the serialization above — saving the RDF from `rudof shex -s examples/person.shex -r turtle` to a file and loading it back with `-f turtle` reproduces the original schema. This is available everywhere a ShEx schema can be loaded: the CLI, the shell (`shex examples/person.shexr.ttl -f turtle`), the MCP server's `show_shex`/`check_shex` tools, and the Python bindings (`read_shex(path, format=ShExFormat.Turtle)`).

## ShEx visualization (UML diagrams)

`rudof shex -r svg` (or `-r png`, `-r plantuml`) renders the schema as a UML class diagram — the same diagram `rudof convert -m shex -x uml -r svg` produces (see [convert: From ShEx to UML](./convert.md#from-shex-to-uml)), but more direct when the schema is already loaded and you don't need `convert`'s other input formats.

```sh
rudof shex -s examples/user.shex -r svg -o user.svg
```

`-r plantuml` writes the intermediate PlantUML source instead of rendering an image — useful to inspect or tweak by hand, or to feed to your own PlantUML setup:

```sh
rudof shex -s examples/user.shex -r plantuml -o user.plantuml
```

`-l`/`--shape-label` scopes the diagram to one shape and its immediate neighbours, same as it does for text output:

```sh
rudof shex -s examples/user.shex -r svg -l :User -o user.svg
```

This uses PlantUML by default; pass `--viz-engine graphviz` to render with Graphviz instead. See [convert: Prerequisites](./convert.md#prerequisites) for what each engine needs installed, and [convert: Choosing a visualization engine](./convert.md#choosing-a-visualization-engine):

```sh
rudof shex -s examples/user.shex -r svg --viz-engine graphviz -o user.svg
```

This also works inside the [shell](./shell.md) — `shex examples/user.shex` to load the schema, then `shex -r svg --viz-engine graphviz -o user.svg`.

## ShEx-based validation

It is also possible to use `rudof` to validate ShEx schemas.

As an example, assuming you have the `user.shex` file as in the previous section and the following `user.ttl` file:

and a file called `user.ttl` with the contents:

```turtle
prefix : <http://example.org/>
prefix schema: <http://schema.org/>

:a schema:name  "Alice" ;
   :status      :Active ;
   schema:knows :a, :b  .

:b schema:name  "Bob"    ;
   :status      :Waiting ;
   schema:knows :c       .

:c schema:name  "Carol"  .

:d schema:name  23      .  # Should fail

:e schema:name  "Emily" ;  # Should fail
   schema:knows :d      .
```

The command runs ShEx validation:

```sh
rudof shex-validate --schema user.shex --node :a --shape-label :User user.ttl
╭──────┬───────┬────────┬───────────────────────╮
│ Node │ Shape │ Status │ Details               │
├──────┼───────┼────────┼───────────────────────┤
│ :a   │ :User │ OK     │ Shape passed :a@:User │
╰──────┴───────┴────────┴───────────────────────╯
```

## Precompiling the schema to a SchemaIR cache

The `--compile-to <FILE>` option runs the AST to IR compilation and writes
the resulting `SchemaIR` to `FILE`. That file can then be reused by
`shex-validate --compiled-schema <FILE>` to skip parsing and
compilation on subsequent runs.

```sh
rudof shex --schema examples/user.shex --compile-to user.ircache
```

See the [precompiled ShEx schemas how-to](../using-rudof/precompiled-shex-schemas.md)
for the full compile - validate workflow.

## ShEx command

The general format of the ShEx subcommand is:

```sh
❯ rudof shex --help
Show information about ShEx schemas

Usage: rudof shex [OPTIONS]

Options:
  -s, --schema <INPUT>            Schema, FILE, URI or - for stdin. If omitted, shows the currently loaded schema
  -f, --format <FORMAT>           Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads, plantuml, svg, png]
  -r, --result-format <FORMAT>    Result schema format [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads, plantuml, svg, png]
      --viz-engine <ENGINE>       Visualization engine for image (SVG/PNG) result formats [default: plantuml] [possible values: plantuml, graphviz]
  -l, --shape-label <LABEL>       shape label
  -t, --show-time <BOOL>          Show processing time [possible values: true, false]
      --show-schema
      --no-show-schema
      --statistics <BOOL>         Show statistics about the schema [possible values: true, false]
  -b, --base <IRI>                Base IRI
      --reader-mode <MODE>        RDF Reader mode (strict or lax) [default: strict] [possible values: lax, strict]
      --show-dependencies <BOOL>  Show dependencies between shapes [possible values: true, false]
      --compile-to <FILE>         Compile the ShEx schema and write the precompiled SchemaIR cache to FILE.
  -c, --config-file <FILE>        Config file name
  -o, --output-file <FILE>        Output file name, default = terminal
      --force-overwrite           Force overwrite to output file if it already exists
  -h, --help                      Print help
```

`--schema` is optional: a bare `rudof shex` shows the schema already loaded in the current session (relevant inside `rudof shell`).

## ShEx configuration file

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in TOML format.

The fields that it can contain are:

- show_extends (Boolean value): If enabled it shows information about extended shapes
- show_imports (Boolean value): If enabled it shows information about imported schemas
- show_shapes (Boolean value): If enabled it shows information about the shapes in the schema
- show_dependencies (Boolean value): If enabled it shows dependencies between shapes
- show_ir (Boolean value): If enabled it shows the ShEx schema's internal representation
- check_well_formed (Boolean value): If enabled it checks the schema meets the ShEx well-formedness requirements
- shex_format (shexc|shexj|turtle|ntriples|rdfxml|trig|n3|nquads|...): Default ShEx format (it can be overridden with the `--format` option)
- base_iri (IRI): Default base declaration to resolve relative IRIs
- rdf (TOML record): Configuration used when the schema format is RDF, following the structure of the `[rdf]` section

The following TOML file can be an example:

```toml
[shex]
show_extends = true
show_imports = true
shex_format = shexc
```
