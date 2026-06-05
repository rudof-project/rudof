//! QLever backend for `rudof_rdf`.
//!
//! Wraps a locally-launched QLever Docker container

mod cli_probe;
mod config;
mod decompressor;
mod error;
mod graph_container;
mod index_builder;
mod runtime;
mod server;

pub use cli_probe::{CliKind, probe as qlever_probe_cli};
pub use config::{InputFile, NativeFormat, QleverConfig};
pub use decompressor::{
    Bzip2Strategy, Compression, CompressionStrategy, DecompressorCandidate, DecompressorProbe, GzipStrategy,
    ResolvedDecompressor, XzStrategy, decompressor_probe, strip_compression_suffix,
};
pub use error::QleverError;
pub use graph_container::QleverGraphContainer;
pub use index_builder::{IndexHandle, build_index};
pub use runtime::runtime;
pub use server::QleverServer;
