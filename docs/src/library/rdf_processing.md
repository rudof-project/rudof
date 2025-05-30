# Processing RDF

The crate [srdf](https://crates.io/crates/srdf) contains several traits and implementations that can be usefult to process RDF.

The architecture of SRDF is based on a set of generic traits with some concrete implementations.

The main traits are:

- [SRDF](https://docs.rs/srdf/latest/srdf/srdf/trait.SRDF.html) contains methods to handle get basic information from RDF graphs required for validating RDF graphs. Mainly get the neighbourhood of RDF nodes (incoming/outgoing arcs, predicates of a subject, triples with some predicate, etc.).
- [SRDFBasic](https://docs.rs/srdf/latest/srdf/srdf_basic/trait.SRDFBasic.html): Types that implement this trait contain basic comparisons and conversions between nodes in RDF graphs
- [SRDFBuilder](https://docs.rs/srdf/latest/srdf/srdf_builder/trait.SRDFBuilder.html): Types that implement this trait can build RDF data.
- [QuerySRDF](https://docs.rs/srdf/latest/srdf/query_srdf/trait.QuerySRDF.html): Types that implement this trait support SPARQL queries.
- [RDFParse](https://docs.rs/srdf/latest/srdf/srdf_parser/trait.RDFParse.html): Represents a generic parser of RDF data inspired by the concept of parser combinators where the input is an RDF graph instead of a sequence of characters. Some parts of this code are inspired by [Combine](https://github.com/Marwes/combine) parser combinators library.

The previous traits are implemented by the following concrete types:

- [SRDFGraph](https://docs.rs/srdf/latest/srdf/srdf_graph/srdfgraph/struct.SRDFGraph.html): implementation of the previous traits based on an in-memory RDF graph using [OxRDF](https://crates.io/crates/oxrdf).
- [SRDFSparql](https://docs.rs/srdf/latest/srdf/srdf_sparql/srdfsparql/struct.SRDFSparql.html): implementation of the previous traits based on an SPARQL endpoint.

> If you want to handle RDF in a generic way, our recommendation is to use only the methods provided by the traits. In that way, your code could work with either in-memory graphs or SPARQL endpoints without having to modify the code.

## Example: Creating an RDF graph

The following code can be used to create a triple in an RDF graph in memory:

```rust
use srdf::SRDFGraph;
use srdf::SRDFBasic;
use iri_s::iri;

let mut graph = SRDFGraph::new();
let alice = <SRDFGraph as SRDFBasic>::iri_s2subject(&iri!("http://example.org/alice"));
let knows = <SRDFGraph as SRDFBasic>::iri_s2iri(&iri!("http://example.org/knows"));
let bob = <SRDFGraph as SRDFBasic>::iri_s2term(&iri!("http://example.org/bob"));

graph.add_triple(&alice, &knows, &bob).unwrap();

assert_eq!(graph.len(), 1);
```
