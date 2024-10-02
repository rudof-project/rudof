# RDF

- [Information about a node](rdf.md#node-information)
- [Merging RDF data](rdf.md#merging-rdf-data)

## Node information

`rudof` can be used to play with RDF data.

For example, it is possible to show information about a Node in an RDF file

It is possible to obtain information about a node in an RDF graph.

Given the following file in `examples/user.shex`:

```turtle
prefix : <http://example.org/>
prefix xsd: <http://www.w3.org/2001/XMLSchema#>

:a :name "Alice" ;
   :birthdate "1990-05-02"^^xsd:date ;
   :enrolledIn :cs101 .

:b :name "Bob", "Robert" .

:cs101 :name "Computer Science" .   
```

If we want to obtain information about the node `:a`, we can run the following command:

```sh
rudof node -n :a examples/simple.ttl 
```

And the result will be:

```txt
Information about node
Outgoing arcs
:a
 -:enrolledIn-> 
      :cs101
 -:birthdate-> 
      "1990-05-02"^^<http://www.w3.org/2001/XMLSchema#date>
 -:name-> 
      "Alice"
```

## Merging RDF data

The `data` option can be used to parse one or more RDF data files, merge them and serialize them to any of the RDF formats supported.

As an example:

```sh
rudof data example/user.ttl example/simple.ttl
```

will parse both RDF data files, merge them and serialize them using the default format (Turtle).

It is possible to serialize the files using a different format, like `ntriples`, `rdfxml`, etc.

For example, the following command will output the result in RDF/XML:

```sh
rudof data example/user.ttl example/simple.ttl -r rdfxml
```
