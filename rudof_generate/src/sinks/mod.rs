//! Output sinks: serializers for the generated graph.
pub mod pg;
#[cfg(not(target_family = "wasm"))]
pub mod rdf;

#[cfg(not(target_family = "wasm"))]
pub use rdf::{GenerationStatistics, OutputWriter};
