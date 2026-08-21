# validate: Generic validate command

The `validate` command is a wrapper on top of `shex-validate` and `shacl-validate` (and, for property graphs, `pgschema-validate`). It takes the same parameters as those commands, plus `--mode` to pick which one to run.

```sh
❯ rudof validate --help
Validate RDF data using ShEx or SHACL

Usage: rudof validate [OPTIONS] [DATA]...

Arguments:
  [DATA]...

Options:
  -M, --mode <MODE>
          Validation mode (ShEx or SHACL)
          [default: shex] [possible values: shex, shacl, pgschema]
  -s, --schema <INPUT>
          Schema used for validation, FILE, URI or - for stdin. If omitted, reuses the currently loaded schema
  -f, --schema-format <FORMAT>
          Schema format
          [default: shexc] [possible values: internal, simple, shexc, shexj, json, jsonld, turtle, ntriples, rdfxml, trig, n3, nquads]
  -m, --shapemap <INPUT>
          ShapeMap used for validation, FILE, URI or - for stdin
      --shapemap-format <FORMAT>
          ShapeMap format
          [default: compact] [possible values: compact, internal, json, details, csv]
      --base-data <IRI>
          Base IRI for data
      --base-schema <IRI>
          Base IRI for Schema
      --sort_by <SORT_MODE>
          Sort result by (default = node)
          [default: node] [possible values: node, details]
  -n, --node <NODE>
          Node to validate
  -l, --shape-label <LABEL>
          shape label (default = START)
  -t, --data-format <FORMAT>
          RDF Data format (default = turtle)
          [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
      --max-steps <NUMBER>
          max steps to run during validation
          [default: 100]
  -S, --shacl-mode <MODE>
          SHACL validation mode (default = native)
          [default: native] [possible values: native, sparql]
      --reader-mode <MODE>
          RDF Reader mode
          [default: strict] [possible values: lax, strict]
  -r, --result-format <FORMAT>
          Ouput result format, default = compact
          [default: compact] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, compact, details, json, csv]
      --map-state <FILE>
          MapState file name
  -c, --config-file <FILE>
          Config file name
  -o, --output-file <FILE>
          Output file name, default = terminal
      --force-overwrite
          Force overwrite to output file if it already exists
      --backend <BACKEND>
          RDF data backend selection: memory | qlever | endpoint=<URL_OR_NAME>
  -e, --endpoint <URL_OR_NAME>
          Shortcut for `--backend endpoint=<URL_OR_NAME>`
  -h, --help
          Print help
```

`--schema` is optional: if you already loaded a schema in the same session (for example inside `rudof shell`), a bare `rudof validate` reuses it. See the [RDF backend (`--backend`) reference](./backend.md) for `--backend`/`--endpoint`.

## Tip: Changing the shapemap in the input

A typical scenario validating RDF with ShEx is to use the same ShEx schema and the same RDF data but trying different shapemaps.
Providing those different shapemap attempts in a file can be boring. One possibility is to use the '-' for the shapemap and `rudof` will expect that the shapemap comes from the stdin.

For example:

```sh
rudof validate -s examples/simple.shex examples/simple.ttl -m -
```

will expect that the shapemap comes from stdin. Once it is typed followed by CTRL-D, the system will ouput the result of the validation.
