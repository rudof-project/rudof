//! Output sinks: serializers for the generated graph.
pub mod pg;
pub mod rdf;

pub use rdf::{GenerationStatistics, OutputWriter};
