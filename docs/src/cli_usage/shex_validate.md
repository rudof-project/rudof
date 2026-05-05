# shex-validate

This command can be used to validate RDF data using ShEx. This is a specific command for ShEx.

## Examples

### Validate data using a ShEx schema and ShapeMap

```sh
$ rudof shex-validate -s examples/user.shex -m examples/user.sm examples/user.ttl
:a@<http://example.org/User> -> OK
:b@<http://example.org/User> -> OK
:c@<http://example.org/User> -> OK
:d@<http://example.org/User> -> Fail
:e@<http://example.org/User> -> Fail
```

### Validate data using ShEx schema, a node and a shape

```sh
$ rudof shex-validate -s examples/user.shex -n ":a" -l ":User" examples/user.ttl
:a@<http://example.org/User> -> OK
:b@<http://example.org/User> -> OK
:c@<http://example.org/User> -> OK
:d@<http://example.org/User> -> Fail
:e@<http://example.org/User> -> Fail
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

Usage: rudof.exe shex-validate [OPTIONS] [DATA]...

Arguments:
  [DATA]...

Options:
  -s, --schema <INPUT>            Schema file name, URI or - (for stdin)
  -f, --schema-format <FORMAT>    ShEx Schema format [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads]
  -m, --shapemap <INPUT>          ShapeMap
      --shapemap-format <FORMAT>  ShapeMap format [default: compact] [possible values: compact, internal, json, details]
  -n, --node <NODE>               Node to validate
      --sort_by <SORT_MODE>       Sort result by (default = node) [default: node] [possible values: node, shape, status, details]
  -l, --shape-label <LABEL>       shape label (default = START)
  -t, --data-format <FORMAT>      RDF Data format [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld]
      --base-schema <IRI>         Base Schema (used to resolve relative IRIs in Schema)
      --base-data <IRI>           Base RDF Data IRI (used to resolve relative IRIs in RDF data)
      --reader-mode <MODE>        RDF Reader mode [default: strict] [possible values: lax, strict]
  -e, --endpoint <NAME>           Endpoint with RDF data (name or URL)
  -r, --result-format <FORMAT>    Ouput result format [default: details] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, compact, details, json]
  -o, --output-file <FILE>        Output file name, default = terminal
  -c, --config-file <FILE>        Config file name
      --force-overwrite           Force overwrite to output file if it already exists
  -h, --help                      Print help
```
