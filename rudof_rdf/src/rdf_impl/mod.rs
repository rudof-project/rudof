mod oxigraph;
#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
mod qlever;

mod backend;
mod backend_error;

pub use oxigraph::{OxigraphInMemory, OxigraphInMemoryError, ReaderMode};
#[cfg(all(not(target_family = "wasm"), feature = "sparql"))]
pub use oxigraph::{OxigraphEndpoint, OxigraphEndpointError, SparqlVars};
#[cfg(all(not(target_family = "wasm"), feature = "qlever"))]
pub use qlever::{
    CliKind, IndexHandle, InputFile, NativeFormat, QleverConfig, QleverError, QleverGraphContainer, QleverServer,
    build_index, qlever_probe_cli, runtime as qlever_runtime,
};

pub use backend::RdfBackend;
pub use backend_error::RdfBackendError;

#[cfg(test)]
mod tests {
    mod in_memory_tests;

    #[cfg(feature = "qlever-docker-tests")]
    mod qlever_docker;
}
