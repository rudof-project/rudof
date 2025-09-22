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

## Example

The following command can be used to compare the shape `http://example.org/Person` in the schemas `examples/shex/compare1.shex` and `examples/shex/compare2.shex`:

```sh
$ rudof compare --schema1 examples/shex/compare1.shex --format1 shexc --mode1 shex --schema2 examples/shex/compare2.shex --format2 shexc --mode2 shex --shape1 "http://example.org/Person" --shape2 "http://example.org/Person"
Shapes Comparison:
 Equal properties:
  - http://example.org/knows: 
  - descr1:   - value: http://example.org/knows
  - datatype: _

  - descr2:   - value: http://example.org/knows
  - datatype: _


  - http://example.org/name: 
  - descr1:   - value: http://example.org/name
  - datatype: _

  - descr2:   - value: http://example.org/name
  - datatype: _


 Properties in shape 1 that are not in shape 2:
  - http://example.org/worksFor: 
  - descr:   - value: http://example.org/worksFor
  - datatype: _


  - http://example.org/age: 
  - descr:   - value: http://example.org/age
  - datatype: _


 Properties in shape 2 that are not in shape 1:
  - http://example.org/email: 
  - descr:   - value: http://example.org/email
  - datatype: _


  - http://example.org/birthDate: 
  - descr:   - value: http://example.org/birthDate
  - datatype: _
```

The output of the comparison can be in compact form (by default) or in JSON using the option `-r json`:

```sh
$ rudof compare --schema1 examples/shex/compare1.shex --format1 shexc --mode1 shex --schema2 examples/shex/compare2.shex --format2 shexc --mode2 shex --shape1 "http://example.org/Person" --shape2 "http://example.org/Person" -r json
{
  "equal_properties": {
    "http://example.org/knows": {
      "description1": {
        "iri_ref": "http://example.org/knows"
      },
      "description2": {
        "iri_ref": "http://example.org/knows"
      }
    },
    "http://example.org/name": {
      "description1": {
        "iri_ref": "http://example.org/name"
      },
      "description2": {
        "iri_ref": "http://example.org/name"
      }
    }
  },
  "properties1": {
    "http://example.org/worksFor": {
      "description": {
        "iri_ref": "http://example.org/worksFor"
      }
    },
    "http://example.org/age": {
      "description": {
        "iri_ref": "http://example.org/age"
      }
    }
  },
  "properties2": {
    "http://example.org/email": {
      "description": {
        "iri_ref": "http://example.org/email"
      }
    },
    "http://example.org/birthDate": {
      "description": {
        "iri_ref": "http://example.org/birthDate"
      }
    }
  }
}
```

### Python

In `Python` there is an equivalent method in the class [Rudof](https://pyrudof.readthedocs.io/en/latest/library.html#rudof) called [`compare_schemas_str`](https://pyrudof.readthedocs.io/en/latest/library.html#pyrudof.Rudof.compare_schemas_str).