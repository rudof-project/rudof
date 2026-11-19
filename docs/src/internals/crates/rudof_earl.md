# `rudof_earl`

[EARL 1.0](https://www.w3.org/TR/EARL10-Schema/) reports, built in memory and serialized as RDF.

EARL is the vocabulary the W3C uses to collect implementation reports: one `earl:Assertion` per test, naming what was tested,
against which criterion, and with what outcome. The [SHACL implementation report page](https://w3c.github.io/data-shapes/data-shapes-test-suite/)
is compiled from EARL files submitted to the `w3c/data-shapes` repository.

## The model

```rust
pub struct EarlReport { /* document metadata, software, assertions */ }
pub struct Assertion   { subject, test, result, asserted_by, extra }
pub struct TestResult  { outcome, mode, info, pointer, date, extra }
pub struct Software    { /* doap:Project + earl:Software + earl:TestSubject */ }
pub struct Release     { /* doap:Version */ }
pub struct Assertor    { /* earl:Assertor, when it needs describing */ }

pub enum Outcome  { Passed, Failed, CantTell, Inapplicable, Untested }
pub enum TestMode { Automatic, Manual, SemiAuto, Undisclosed, UnknownMode }
```

### `extra`: domain terms without a domain dependency

`TestResult` and `Assertion` each carry an `extra: Vec<(IriS, Object)>`, written as plain triples on the node.

### Document metadata

`dct:issued`, `dct:title`, `foaf:maker` and `foaf:primaryTopic` describe the report document itself, which needs
a subject — so they are only written once the document is named with `EarlReport::with_document`.
Turtle's `<>` is an empty relative IRI, not something a graph can hold, so pass the IRI the report will be published under.

## Dependencies

- Internal: [`rudof_rdf`](./rudof_rdf.md), [`rudof_iri`](./rudof_iri.md), [`prefixmap`](./prefixmap.md)
- External: `const_format`, `paste`, `thiserror`

## Documentation

The crate documentation can be found [here](https://docs.rs/rudof_earl).
