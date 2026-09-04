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

## Checking whether shapes are recursive

`-r internal` shows `rudof`'s internal representation of the shapes graph, including a
"Recursive shapes" section that classifies every shape that takes part in a cyclic shape
reference:

```sh
rudof shacl -s recursive_shapes.ttl -r internal
```

Each recursive shape is reported as one of:

- **positive recursive** — the cycle uses only monotonic constraints (`sh:and`, `sh:or`,
  `sh:node`, `sh:property`, ...); supported under both `cautious` and `brave`.
- **stratified recursive** — the cycle also carries a negating constraint (`sh:not`,
  `sh:xone`, ...), but it targets a shape outside of any recursion, so it can be resolved
  independently; supported under both `cautious` and `brave`.
- **non-stratified recursive** — a negating constraint in the cycle reaches back into a
  recursive shape (its own, or a different one's); no safe order exists to resolve it, so a
  schema like this is always rejected, at `-s`/`--shapes` load time.

Shapes that aren't part of any cycle are omitted from this section (or the whole section
reads "none" if the schema has no recursion at all). See
[Recursive shapes](./shacl_validate.md#recursive-shapes) for the full explanation and how
`--recursion-semantics` picks between `cautious` and `brave`.

## Selecting the RDF backend

By default, SHACL data is loaded into an in-process `memory` graph. Use `--backend` to switch to a QLever Docker container or a remote SPARQL endpoint:

```sh
rudof shacl -s shapes.ttl --backend qlever data.ttl
rudof shacl -s shapes.ttl --endpoint https://my.sparql.server/sparql
```

See the [RDF backend (`--backend`) reference](./backend.md) for full documentation.
