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

Usage: rudof service [OPTIONS] --service <SPARQL service name>

Options:
  -s, --service <SPARQL service name>
          
  -f, --format <SPARQL service format>
          [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads]
  -o, --output-file <Output file name, default = terminal>
          
  -r, --result-format <Result service format>
          [default: internal] [possible values: internal]
      --reader-mode <RDF Reader mode>
          RDF Reader mode [default: strict] [possible values: lax, strict]
  -c, --config-file <Config file name>
          Config file path, if unset it assumes default config
      --force-overwrite
          
  -h, --help
          Print help
```

## Service config file

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in TOML format.

The fields that it can contain are:

- base (IRI): Base IRi to resolve relative IRIs in the service description.
