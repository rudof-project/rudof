# config

`rudof config` prints the effective configuration as TOML: built-in defaults merged with the user config file and any `rudof.toml` found between the filesystem root and the current directory (or just the file passed via `--config-file`, if given). See [Configuration](../general/configuration.md) for how these sources are merged.

```sh
rudof config
```

```toml
version = "0.3.9"
auto_base = false

[rdf]
local_base = true

[rdf.endpoints.wikidata]
query_url = "https://query.wikidata.org/sparql"
# ...
```

`[rdf.endpoints.*]` lists the SPARQL endpoints registered by name, the ones usable as `--endpoint wikidata` or `--backend endpoint=wikidata`. `wikidata`, `dbpedia` and `uniprot` are registered by default.

## Inspecting a specific config file

```sh
rudof config -c my-rudof.toml
```

## Options

```sh
Usage: rudof config [OPTIONS]

Options:
  -c, --config-file <FILE>  Config file name
  -o, --output-file <FILE>  Output file name, default = terminal
      --force-overwrite     Force overwrite to output file if it already exists
  -h, --help                Print help
```
