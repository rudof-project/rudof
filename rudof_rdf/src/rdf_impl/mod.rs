mod oxigraph;
#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
mod qlever;

mod backend;
mod backend_error;

#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
pub use oxigraph::{OxigraphEndpoint, OxigraphEndpointError, SparqlVars};
pub use oxigraph::{OxigraphInMemory, OxigraphInMemoryError, ReaderMode};
#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
pub use qlever::{
    Bzip2Strategy, CliKind, Compression, CompressionStrategy, DecompressorCandidate, DecompressorProbe, IndexHandle,
    InputFile, NativeFormat, QleverConfig, QleverError, QleverGraphContainer, QleverServer, ResolvedDecompressor,
    XzStrategy, build_index, decompressor_probe, qlever_probe_cli, runtime as qlever_runtime, strip_compression_suffix,
};

pub use backend::RdfBackend;
pub use backend_error::RdfBackendError;

#[cfg(test)]
mod tests {
    mod in_memory_tests;

    #[cfg(feature = "qlever-docker-tests")]
    mod qlever_docker;
}
