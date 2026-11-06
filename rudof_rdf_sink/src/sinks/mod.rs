mod build_rdf;
mod counting;
#[cfg(all(feature = "sparql-update", not(target_family = "wasm")))]
mod sparql_update;
mod writer;

pub use build_rdf::{BuildRdfSink, GraphSink};
pub use counting::CountingSink;
#[cfg(all(feature = "sparql-update", not(target_family = "wasm")))]
pub use sparql_update::{DEFAULT_BATCH_TRIPLES, SparqlUpdateSink};
pub use writer::WriterSink;
