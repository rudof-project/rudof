# Information about RDF nodes

`rudof` can be used to play with RDF data.
For example, it is possible to show information about a an RDF node in a graph.
Given the following RDF graph, that can be obtained using `curl`.

```turtle
:a :name       "Alice"                  ;
   :birthdate  "1990-05-02"^^xsd:date   ;
   :enrolledIn :cs101                   .

:b :name "Bob", "Robert" .

:cs101 :name "Computer Science" .
```

```sh
curl -o simple.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple.ttl
```

We can obtain information about the node `:a` (or any other node) by running the following command:

```sh
rudof node -n :a simple.ttl 
```

> Note that the `-n` argument is used to provide the focus node by indicating the node's URI.
