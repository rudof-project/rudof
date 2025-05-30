# validate: Generic validate command

The `validate` command is a wrapper on top of `shex-validate` and `shacl_validate` which contain the same parameters but adds the parameter: `--mode` to indicate eithre `shex` or `shacl`.

```sh
❯ rudof validate --help
Validate RDF data using ShEx or SHACL

Usage: rudof validate [OPTIONS] --schema <Schema file name> [DATA]...

Arguments:
  [DATA]...  

Options:
  -M, --mode <Validation mode>
          [default: shex] [possible values: shex, shacl]
  -s, --schema <Schema file name>
          
  -f, --schema-format <Schema format>
          [default: shexc] [possible values: internal, simple, shexc, shexj, turtle, ntriples, rdfxml, trig, n3, nquads]
  -m, --shapemap <ShapeMap file name>
          
      --shapemap-format <ShapeMap format>
          [default: compact] [possible values: compact, internal]
  -n, --node <NODE>
          
  -l, --shape-label <shape label (default = START)>
          
  -t, --data-format <RDF Data format>
          [default: turtle] [possible values: turtle, ntriples, rdfxml, trig, n3, nquads]
  -e, --endpoint <Endpoint with RDF data>
          
      --max-steps <max steps to run>
          [default: 100]
  -S, --shacl-mode <SHACL validation mode>
          Execution mode [default: default] [possible values: default, sparql]
      --reader-mode <RDF Reader mode>
          RDF Reader mode [default: strict] [possible values: lax, strict]
  -o, --output-file <Output file name, default = terminal>
          
      --force-overwrite
          
  -h, --help
          Print help
```

## Tip: Changing the shapemap in the input

A typical scenario validating RDF with ShEx is to use the same ShEx schema and the same RDF data but trying different shapemaps. 
Providing those different shapemap attempts in a file can be boring. One possibility is to use the '-' for the shapemap and `rudof` will expect that the shapemap comes from the stdin. 

For example:

```sh
rudof validate -s examples/simple.shex examples/simple.ttl -m -
```

will expect that the shapemap comes from stdin. Once it is typed followed by CTRL-D, the system will ouput the result of the validation.
