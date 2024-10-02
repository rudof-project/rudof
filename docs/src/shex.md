# ShEx

## Infor about ShEx schema

Read a ShEx schema in compact syntax and show its JSON representation:

```sh
rudof shex -s examples/user.shex
```

### General help for schema command

```sh
$ rudof shex --help
Usage: rudof shex [OPTIONS] --schema <Schema file name>
Options:
  -s, --schema <Schema file name>
  -f, --schema-format <Schema format>
          [default: shexc] [possible values: internal, shexc, shexj, turtle]
  -r, --result-schema-format <Result schema format>
          [default: shexj] [possible values: internal, shexc, shexj, turtle]
  -o, --output-file <Output file name, default = terminal>
  -h, --help
          Print help
```

## ShEx validation

It is also possible to use `rudof` to validate ShEx schemas using the following:

```sh
rudof validate --data examples/user.ttl --schema examples/user.shex --node :a --shape-label :User
```
