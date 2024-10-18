# service: Get information about a SPARQL endpoint service

This command can be used to get information from the description provided by a SPARQL endpoint.

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
