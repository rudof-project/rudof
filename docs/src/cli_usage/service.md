# service: Get information about a SPARQL endpoint service

This command can be used to get information from the service description provided by a SPARQL endpoint. It is based on the [SPARQL 1.1 Service description vocabulary](https://www.w3.org/TR/sparql11-service-description/) which is W3C recommendation that describes the features that a SPARQL endpoint supports.

As an example, to obtain information about Uniprot you can use:

```sh
❯ rudof service -s https://sparql.uniprot.org/sparql
Service
  endpoint: https://sparql.uniprot.org/sparql
  supportedLanguage: [SPARQL11Query]
  feature: [UnionDefaultGraph, BasicFederatedQuery]
  result_format: [JSON, CSV, TSV, N-TRIPLES, Turtle, RDF/XML, XML]
  default_dataset: Dataset: base://#_1
```

## Service command options

The full command options are:

```sh
Show information about SPARQL service

Usage: rudof service [OPTIONS]

Options:
  -s, --service <URL>           SPARQL service URL. If omitted, shows the currently loaded service description
  -f, --format <FORMAT>         SPARQL service format [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads, jsonld, pg]
  -r, --result-format <FORMAT>  Output result service format [default: json] [possible values: internal, mie, json]
      --reader-mode <MODE>      RDF Reader mode [default: strict] [possible values: lax, strict]
      --base <IRI>              Base used to resolve relative IRIs in the service description
  -c, --config-file <FILE>      Config file name
  -o, --output-file <FILE>      Output file name, default = terminal
      --force-overwrite         Force overwrite to output file if it already exists
  -h, --help                    Print help
```

`--service` is optional: a bare `rudof service` shows the service description already loaded in the current session (relevant inside `rudof shell`).

## Service config file

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in TOML format.

The fields that it can contain are:

- base_iri (IRI): Base IRI to resolve relative IRIs in the service description.
