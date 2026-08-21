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
  "start": ":User",
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
  -f, --format <FORMAT>           Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads]
  -r, --result-format <FORMAT>    Result schema format [default: shexj] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads]
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
