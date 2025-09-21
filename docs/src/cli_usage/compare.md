# compare: Compare Shapes 

`rudof` supports comparison between different schemas and shapes.

The `compare` has the following structure:

```
$ rudof compare --help
Compare two shapes (which can be in different formats)

Usage: rudof compare [OPTIONS] --schema1 <INPUT> --schema2 <INPUT>

Options:
  -c, --config <FILE>           Path to config file
      --mode1 <MODE>            Input mode first schema [default: shex] [possible values: shacl, shex,
                                dctap, service]
      --mode2 <MODE>            Input mode second schema [default: shex] [possible values: shacl, shex,
                                dctap, service]
      --force-overwrite         Force overwrite to output file if it already exists
      --schema1 <INPUT>         Schema 1 (URI, file or - for stdin)
      --schema2 <INPUT>         Schema 2 (URI, file or - for stdin)
      --format1 <FORMAT>        File format 1 [default: shexc] [possible values: shexc, shexj, turtle]
      --format2 <FORMAT>        File format 2 [default: shexc] [possible values: shexc, shexj, turtle]
  -r, --result-format <FORMAT>  Result format [default: internal] [possible values: internal, json]
  -o, --output-file <FILE>      Output file name, default = terminal
  -t, --target-folder <FOLDER>  Target folder
      --shape1 <LABEL>          shape1 (default = START)
      --shape2 <LABEL>          shape2 (default = START)
      --reader-mode <MODE>      RDF Reader mode [default: strict] [possible values: lax, strict]
      --show-time <SHOW_TIME>   Show processing time [possible values: true, false]
  -h, --help                    Print help
```
