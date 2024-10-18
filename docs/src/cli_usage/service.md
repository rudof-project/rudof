# service: Get information about a SPARQL endpoint service

This command can be used to get information from the description provided by a SPARQL endpoint.

As an example, let's assume one wants to obtain information about Uniprot:

The first step is to obtain the service description in Turtle. It can be done with the following command:

```sh
❯ curl -H "Accept: text/turtle" https://sparql.uniprot.org/sparql > uniprot.ttl
```

And then, `rudof` can be used to parse that file as:

```sh
❯ rudof service -s uniprot.ttl
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

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in YAML format.

The fields that it can contain are:

- base (IRI): Base IRi to resolve relative IRIs in the service description.
