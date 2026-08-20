# rdf-config

The `rdf-config` command reads a schema description written in the [rdf-config](https://github.com/dbcls/rdf-config) YAML DSL.

```sh
rudof rdf-config -s schema.yaml
```

## Options

```sh
Usage: rudof rdf-config [OPTIONS] --source-file <INPUT>

Options:
  -s, --source-file <INPUT>     Source file name (URI, file or - for stdin)
  -r, --result-format <FORMAT>  Output result rdf-config format [default: internal] [possible values: internal, yaml]
  -f, --format <FORMAT>         rdf-config format [default: yaml] [possible values: yaml]
  -c, --config-file <FILE>      Config file name
  -o, --output-file <FILE>      Output file name, default = terminal
      --force-overwrite         Force overwrite to output file if it already exists
  -h, --help                    Print help
```
