# `rudof_rdf_sink`

Where the RDF triples rudof produces end up. A producer (the data generator, a validation report, an RDF-to-RDF transformation) pushes triples into a sink and stops caring what happens to them; the caller decided that when it picked the sink.

## The trait

```rust
pub trait RdfSink {
    fn declare_prefix(&mut self, alias: &str, iri: &IriS) -> Result<(), RdfSinkError>;
    fn declare_prefixes(&mut self, prefixes: PrefixMap) -> Result<(), RdfSinkError>;
    fn declare_base(&mut self, base: &IriS) -> Result<(), RdfSinkError>;
    fn push(&mut self, triple: oxrdf::Triple) -> Result<(), RdfSinkError>;
    fn flush(&mut self) -> Result<(), RdfSinkError>;
    fn finish(&mut self) -> Result<SinkReport, RdfSinkError>;
    fn report(&self) -> SinkReport;
}
```

### `flush` and `finish`

`flush` means *everything pushed so far has reached the destination*; `finish` means *there will be no more triples*, and does what `flush` would have done before closing and reporting.

| | `flush` | `finish` |
| --- | --- | --- |
| times it may be called | any | once |
| after it | the sink takes more triples | `push` and `finish` fail with `Finished` |
| returns | `()` | `SinkReport` |
| `BuildRdfSink` | no-op: the graph *is* the destination | closes, reports |
| `CountingSink` | no-op: nothing is held | closes, reports |
| `WriterSink` | flushes the writer | flushes, closes, reports |
| `SparqlUpdateSink` | sends the partial batch now | sends the tail, closes, reports |

### Declarations

A declaration is dropped where the destination has no syntax for it, and that is `Ok`, not a failure.

| | Result |
| --- | --- |
| a prefix in `CountingSink`, `SparqlUpdateSink`, or an N-Triples/N-Quads `WriterSink` | `Ok`, dropped |
| a prefix or base in a Turtle/TriG `WriterSink`, before the first triple | `Ok`, written in the prelude |
| a prefix or base in a Turtle/TriG `WriterSink`, after the first triple | `Err(DeclaredTooLate { what })` |
| any declaration after `finish`, in all four | `Err(Finished)` |

### `SinkReport`

```rust
pub struct SinkReport {
    pub triples: u64,
}
```

Triples **pushed**, duplicates included (a streaming sink cannot tell a duplicate from a new triple without keeping the graph it exists to avoid keeping).

Anything richer about a kept graph is available through the graph itself: `BuildRdfSink::graph()` hands back the whole `BuildRDF`, so `len()`, `triples()`, `serialize` and SPARQL are all there without the sink restating them.

## The sinks

### `BuildRdfSink<B>` - accumulate, and hand the graph back

```rust
pub type GraphSink = BuildRdfSink<OxigraphInMemory>;
```

The default. Generic over `B: BuildRDF`, which covers two of the behaviours at once: the in-memory graph, and *forwarding to a triple store through its native API*.

Inherent, outside the trait: `new`, `graph`, `graph_mut`, `into_graph`, and, behind the `sparql-update` feature, `upload_to`.

`upload_to` reads the graph into a `SparqlUpdateSink`, so it reuses the sink rather than duplicating it; it consumes neither the sink nor the graph, and works whether or not the sink is finished.

### `CountingSink` - discard, keep the count

Holds the counter and the finished flag and nothing else. The "count them and throw them away" case, and what to reach for when the graph does not fit.

### `WriterSink<W: Write>` - stream straight out

N-Triples by default; `with_format` takes N-Quads, Turtle or TriG as well.

The four formats are the ones oxttl can serialize incrementally through its `low_level()` API, which takes the writer **per call** rather than owning it. RDF/XML and JSON-LD only have the writer-owning API, and N3 has no serializer at all, so `with_format` refuses all three up front with `NotStreamable`.

Turtle and TriG are not excluded for needing the whole graph: `oxttl`'s serializer tracks the current subject and predicate, so it groups consecutive triples with `;` and `,` as they arrive. Only the optimal nesting (`[ ]`, collections) would need the whole graph. The cost is that the last statement stays open between pushes:

| | after a `flush` | after `finish` |
| --- | --- | --- |
| N-Triples, N-Quads | a complete, readable document | the same |
| Turtle, TriG | bytes are on the writer, but the document ends mid-statement | closed and readable |

Prefixes and the base go in the prelude, which is written with the first triple — hence `DeclaredTooLate` for anything declared after it.

### `SparqlUpdateSink` - batched SPARQL Update

Behind the `sparql-update` feature, and native only. `push` appends to a batch and sends it once full; `flush` sends a short batch; `finish` sends the tail. The body is an `INSERT DATA` whose triples are written in N-Triples syntax with absolute IRIs. The sink does that conversion, so a producer only ever hands it typed `oxrdf::Triple` values.

Two things to know:

- **Batching splits blank node scope.** Each `INSERT DATA` mints fresh labels for the blank nodes it mentions, so a blank node appearing in two batches becomes two nodes in the store. To keep a structure in one scope: `with_batch_size(usize::MAX)`, which sends everything in one request at `finish`, or a large batch size plus a `flush` at the producer's own boundary — per entity, per validation result.
- RDF 1.2 triple terms are written `<<( ... )>>`, which a SPARQL 1.1 endpoint rejects. Pushing them is the caller's call.

## Dependencies

- Internal: [`rudof_rdf`](./rudof_rdf.md), [`rudof_iri`](./rudof_iri.md), [`prefixmap`](./prefixmap.md)
- External: `oxrdf`, `oxttl`, `thiserror`, `tracing`

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_rdf_sink).
