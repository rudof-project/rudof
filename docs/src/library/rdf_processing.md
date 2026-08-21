# Processing RDF

The crate [rudof_rdf](https://crates.io/crates/rudof_rdf) contains several traits and implementations that can be useful to process RDF.

The architecture of rudof RDF is based on a set of generic traits, in [`rudof_rdf::rdf_core`](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/index.html), with concrete implementations in [`rudof_rdf::rdf_impl`](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_impl/index.html).

The main traits are:

- [Rdf](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/trait.Rdf.html): the base trait. Defines the associated types shared by every implementation (`Subject`, `IRI`, `Term`, `BNode`, `Literal`, `Triple`) and basic conversions/comparisons between them.
- [NeighsRDF](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/trait.NeighsRDF.html): extends `Rdf` with methods to get basic information from RDF graphs required for validating RDF graphs. Mainly get the neighbourhood of RDF nodes (incoming/outgoing arcs, predicates of a subject, triples with some predicate, etc.), and follow SHACL property paths.
- [BuildRDF](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/trait.BuildRDF.html): extends `NeighsRDF` with methods to build RDF data — add/remove triples, manage prefixes and the base IRI, and serialize the graph.
- [FocusRDF](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/trait.FocusRDF.html): extends `NeighsRDF` with a "focus node", a current point of reference in the graph that parsing operations (e.g. SHACL/ShEx validation) can navigate from without repeatedly specifying it.
- [QueryRDF](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/query/trait.QueryRDF.html): extends `Rdf` with SPARQL query support (SELECT, CONSTRUCT, ASK).
- [AsyncRDF](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/trait.AsyncRDF.html): an async counterpart of the above, used by backends whose work is inherently asynchronous (Docker, HTTP), such as the QLever backend.
- [RDFNodeParse](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_core/parser/rdf_node_parser/trait.RDFNodeParse.html): a generic parser of RDF data inspired by the concept of parser combinators, where the input is an RDF node instead of a sequence of characters. Some parts of this code are inspired by the [Combine](https://github.com/Marwes/combine) parser combinators library — see [Parsing RDF](./parsing_processing.md) for more.

The previous traits are implemented by the following concrete types, in [`rudof_rdf::rdf_impl`](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_impl/index.html):

- [OxigraphInMemory](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_impl/struct.OxigraphInMemory.html): implementation of the previous traits based on an in-memory RDF graph using [oxrdf](https://crates.io/crates/oxrdf). This is what `data`, `shex`, `shacl`, etc. use by default.
- [OxigraphEndpoint](https://docs.rs/rudof_rdf/latest/rudof_rdf/rdf_impl/struct.OxigraphEndpoint.html): implementation based on a remote SPARQL endpoint.
- `QleverGraphContainer` (behind the `qlever` feature): implementation backed by a local [QLever](https://github.com/ad-freiburg/qlever) Docker container. See the [`--backend` reference](../cli_usage/backend.md) for how backends are selected from the CLI.

> If you want to handle RDF in a generic way, our recommendation is to write your code against the traits (`Rdf`, `NeighsRDF`, `BuildRDF`, ...) rather than a concrete type. In that way, your code could work with an in-memory graph, a SPARQL endpoint, or the QLever backend without having to modify the code.

## Example: Creating an RDF graph

The following code can be used to create a triple in an RDF graph in memory:

```rust
use rudof_iri::iri;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_impl::OxigraphInMemory;

let mut graph = OxigraphInMemory::new();
let alice = iri!("http://example.org/alice");
let knows = iri!("http://example.org/knows");
let bob = iri!("http://example.org/bob");

graph.add_triple(alice, knows, bob).unwrap();

assert_eq!(graph.len(), 1);
```
