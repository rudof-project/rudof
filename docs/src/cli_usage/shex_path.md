# shex-path

The command `shex-path` evaluates a [ShExPath](https://shexspec.github.io/spec/ShExPath) against a ShEx schema and prints the elements of the schema it selects. A ShExPath addresses shape expressions, triple constraints, value sets, annotations and the other elements of a schema: `@:Person~:knows` is the `:knows` triple constraint of the shape `:Person`.

The path is written with the prefixes and base the schema declares, and it is evaluated over the ShExJ form of the schema. Each selected element is printed as a JSON object with its `location`, a [JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901) into that ShExJ document, and its `value`, the element as ShExJ. The elements come in the order the path selects them, with their duplicates.

See [ShExPath](../using-rudof/shexpath.md) for the language.

## Examples

### Select a triple constraint

```sh
$ rudof shex-path -s "PREFIX : <http://example.org/> :Person { :name . ; :knows @:Person * }" \
    -p "@:Person~:knows"
[
  {
    "location": "/shapes/0/shapeExpr/expression/expressions/1",
    "value": {
      "max": -1,
      "min": 0,
      "predicate": "http://example.org/knows",
      "type": "TripleConstraint",
      "valueExpr": "http://example.org/Person"
    }
  }
]
```

### Select an atomic value

A reference to a shape is an IRI, not the shape it names:

```sh
$ rudof shex-path -s "PREFIX : <http://example.org/> :Person { :name . ; :knows @:Person * }" \
    -p "@:Person~:knows/valueExpr"
[
  {
    "location": "/shapes/0/shapeExpr/expression/expressions/1/valueExpr",
    "value": "http://example.org/Person"
  }
]
```

### A path that selects nothing

```sh
$ rudof shex-path -s "PREFIX : <http://example.org/> :Person { :name . }" -p "@:Nope"
[]
```

### A failed assertion

`[assert ...]` makes the path an error when the assertion does not hold. The error is written to the standard error, and the command exits with a non-zero code:

```sh
$ rudof shex-path -s "PREFIX : <http://example.org/> :Person { :name . ; :knows @:Person * }" \
    -p "@:Person/expression/expressions/*[assert count() = 1]"
Error: ShEx error: Failed to evaluate the ShExPath '@:Person/expression/expressions/*[assert count() = 1]': ShExPath assertion failed on the item at "/shapes/0/shapeExpr/expression/expressions/0"
```

A path that is not a ShExPath is reported the same way.

## Options

| Option | Description |
|---|---|
| `-s, --schema INPUT` | Schema, file, URI, or `-` for stdin (required) |
| `-p, --path SHEXPATH` | ShExPath to evaluate, written with the prefixes and base of the schema (required) |
| `-f, --format FORMAT` | Schema format (ShExC, ShExJ, Turtle, ...), default `ShExC` |
| `-b, --base IRI` | Base IRI for resolving relative IRIs in the schema |
| `-c, --config-file FILE` | Config file name |
| `-o, --output-file FILE` | Output file name, default terminal |
| `--force-overwrite` | Overwrite the output file if it already exists |
