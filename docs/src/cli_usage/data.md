# Information about RDF data

## Merging RDF data

The `data` option can be used to parse one or more RDF data files, merge them and serialize them to any of the RDF formats supported.
Given the following `turtle` files:

```sh
curl -o simple.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple.ttl
curl -o user.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/user.ttl
```

We will parse both files, merge and serialize them in RDF/XML.

> It is possible to serialize the files using a different format, like `ntriples`, `rdfxml`, etc.

```sh
rudof data user.ttl simple.ttl -r rdfxml >> output.rdf
```

> In this example we are piping the result to a file, but you can always print it to the terminal by omitting the `>> output.rdf` declaration.

## RDF Config file

The parameter `--config-file`  (`-c` in short form) can be used to pass a configuration file in YAML format.

The fields that it can contain are:

- base (IRI): Default base declaration to resulve relative IRIs
