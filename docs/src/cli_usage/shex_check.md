# shex-check

The command `shex-check` checks whether a ShEx schema is well-formed: it parses the schema, compiles it, and checks for negative dependency cycles. Unlike `shex`, it never loads the schema into session state, and unlike `shex-validate`, it needs no RDF data or ShapeMap — it only checks the schema itself.

## Examples

### Check a well-formed schema

```sh
$ rudof shex-check -s examples/user.shex
Schema is valid: well-formed and contains no negative cycles.
```

### Check a schema with a negative cycle

```sh
$ rudof shex-check -s "PREFIX : <http://example.org/> :S { :p @:T } :T { :q NOT @:S }"
Schema contains negative cycles in its dependency graph:

Negative cycle #1:
  Shapes involved:
    - :S
    - :T

  Negative cycle path:
    :S <--[NOT]-- :T <-- :S
```

`shex-check` always exits successfully (exit code `0`) whether or not the schema is well-formed — the report itself communicates the result, the same way `shex-validate`'s validation report does for data conformance.

## Options

| Option | Description |
|---|---|
| `-s, --schema INPUT` | Schema, file, URI, or `-` for stdin (required) |
| `-f, --format FORMAT` | Schema format (ShExC, ShExJ, Turtle, ...), default `ShExC` |
| `-b, --base IRI` | Base IRI for resolving relative IRIs in the schema |
