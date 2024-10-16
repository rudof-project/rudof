# SHACL

[SHACL](https://www.w3.org/TR/shacl/) is the W3C Recommendation for validating RDF data.
That's why we have decided to provide some features that would help SHACL developers do some typical workflows.
For following the examples please download the following [file](https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple_shacl.ttl) from the Github repository.

> The provided file contains a simple shapes graph that can be used for executing the different examples that are included in this page.

```sh
curl -o simple_shacl.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple_shacl.ttl
```

## Describe a SHACL graph

You can describe a shapes graph stored in a file by executing the `shacl` command.
As a result, a message with all the information associated with each shape inside the graph are going to be prompted in the terminal.

```sh
rudof shacl -s simple_shacl.ttl
```

## Convert from one format to another

It is also possible to read a SHACL shapes graph and convert it to some format
In the example below, `rudof` will read a SHACL file in Turle and convert it to RDF/XML.

> In this example we are piping the result to a file, but you can always print it to the terminal by omitting the `>> output.rdf` declaration.

```sh
rudof shacl -s simple_shacl.ttl -r rdfxml >> output.rdf
```