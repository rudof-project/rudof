//! Where the RDF triples rudof produces end up.

mod error;
mod report;
mod sink;
mod sinks;
mod triples_sink;

pub use error::RdfSinkError;
pub use report::SinkReport;
pub use sink::RdfSink;
pub use sinks::{BuildRdfSink, CountingSink, GraphSink, WriterSink};
#[cfg(all(feature = "sparql-update", not(target_family = "wasm")))]
pub use sinks::{DEFAULT_BATCH_TRIPLES, SparqlUpdateSink};
pub use triples_sink::TriplesSink;
