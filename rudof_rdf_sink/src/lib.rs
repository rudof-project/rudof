//! Where the RDF triples rudof produces end up.

mod error;
mod report;
mod sink;
mod sinks;

pub use error::RdfSinkError;
pub use report::SinkReport;
pub use sink::RdfSink;
pub use sinks::{BuildRdfSink, CountingSink, GraphSink, WriterSink};
