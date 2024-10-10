# Getting started

`rudof` is a tool to process and validate RDF data using shapes, as well as converting between different RDF data models.

As a command line tool, it contains several subcommands which can be decomposed in two types:

- Commands about some technology, which take some input from that technology and provide information about it. Examples are: `shex`, `shacl`, `dctap`, `shapemap`, `service`, `node` and `data`.
- Commands that do some actions like: `query`, `validate` or `convert`.

```sh
$ rudof help
A tool to process and validate RDF data using shapes, and convert between different RDF data models

Usage: rudof [OPTIONS] [COMMAND]

Commands:
  shapemap        Show information about ShEx ShapeMaps
  shex            Show information about ShEx schemas
  validate        Validate RDF data using ShEx or SHACL
  shex-validate   Validate RDF using ShEx schemas
  shacl-validate  Validate RDF data using SHACL shapes
  data            Show information about RDF data
  node            Show information about a node in an RDF Graph
  shacl           Show information about SHACL shapes
  dctap           Show information and process DCTAP files
  convert         Convert between different Data modeling technologies
  service         Show information about SPARQL service
  query           Run SPARQL queries
  help            Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...
          

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
