//! Where the RDF triples rudof produces end up.

mod error;
mod report;
mod sink;

pub use error::RdfSinkError;
pub use report::SinkReport;
pub use sink::RdfSink;
