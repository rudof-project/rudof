# SHACL

[SHACL](https://www.w3.org/TR/shacl/) is the W3C Recommendation for validating RDF data.
That's why we have decided to provide some features that would help SHACL developers do some typical workflows.

The SHACL command can be used to obtain information about a SHACL shapes graph.

Assuming that a file `simple_shacl.ttl` contains the following data:

```turtle
@prefix :       <http://example.org/> .
@prefix sh:     <http://www.w3.org/ns/shacl#> .
@prefix xsd:    <http://www.w3.org/2001/XMLSchema#> .
        
:Person a sh:NodeShape;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] ;
  sh:property [                   
   sh:path     :birthDate ; 
   sh:maxCount 1; 
   sh:datatype xsd:date ;
  ] ;
  sh:property [                   
   sh:path     :enrolledIn ; 
   sh:node    :Course ;
  ] .
:Course a sh:NodeShape;
   sh:closed true ;
   sh:property [                  
    sh:path     :name ; 
    sh:minCount 1; 
    sh:maxCount 1;
    sh:datatype xsd:string ;
  ] .
```

The file can also obtained from the [examples/simple_shacl.ttl](https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple_shacl.ttl).

```sh
curl -o simple_shacl.ttl https://raw.githubusercontent.com/rudof-project/rudof/refs/heads/master/examples/simple_shacl.ttl
```

The following command can be used to get information about a SHACL shapes graph:

```sh
rudof shacl -s simple_shacl.ttl
```

## Convert from one format to another

It is also possible to read a SHACL shapes graph and convert it to some format

In the example below, `rudof` will read a SHACL file in Turle and convert it to RDF/XML.

```sh
rudof shacl -s simple_shacl.ttl -r rdfxml -o output.rdf
```
