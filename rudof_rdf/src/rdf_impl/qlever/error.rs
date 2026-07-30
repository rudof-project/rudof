//! Errors returned by the QLever backend.

use oxiri::IriParseError;
use oxrdf::Term;
use rudof_iri::error::IriSError;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

use crate::rdf_impl::OxigraphEndpointError;

/// Errors returned by [`QleverGraphContainer`](super::QleverGraphContainer) and
/// the helpers that build / serve QLever indexes.
#[derive(Error, Debug)]
pub enum QleverError {
    /// The Docker daemon is not reachable.
    #[error(
        "Docker daemon is not reachable. The QLever backend needs a running Docker daemon. Underlying error: {message}"
    )]
    DockerUnreachable { message: String },

    /// A bollard call against the Docker daemon failed.
    #[error("Docker API error: {0}")]
    Bollard(#[from] bollard::errors::Error),

    /// A testcontainers call failed.
    #[error("Container error: {0}")]
    Testcontainers(#[from] testcontainers::TestcontainersError),

    /// The QLever container exited with a non-zero status while running a one-shot command (typically index building).
    #[error("Container exit {status} running {what}\nlogs:\n{logs}")]
    ContainerNonZeroExit { what: String, status: i64, logs: String },

    /// The server failed to become responsive within the configured timeout.
    #[error("QLever server at {endpoint} did not respond within {timeout_secs}s")]
    ServerStartupTimeout { endpoint: String, timeout_secs: u64 },

    /// The on-disk index directory could not be created / accessed.
    #[error("I/O error on index dir {path}: {error}")]
    IndexDirIo {
        path: PathBuf,
        #[source]
        error: io::Error,
    },

    /// Generic filesystem error (input files, temp files, …).
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    /// Pre-flight check failed (image missing, bind dir not writable, …).
    #[error("QLever pre-flight check failed: {0}")]
    PreFlight(String),

    /// We probed the image and could not figure out which CLI it exposes.
    #[error(
        "Could not detect QLever CLI version. Neither `IndexBuilderMain -h` nor `qlever-index -h` succeeded against image {image}"
    )]
    UnknownCliKind { image: String },

    /// Tried to convert a non-Turtle/N-Triples/N-Quads input but the conversion failed.
    #[error("Format conversion failed for input {source_name}: {error}")]
    FormatConversion { source_name: String, error: String },

    /// Wrapped error coming out of the underlying SPARQL-over-HTTP layer.
    #[error(transparent)]
    Sparql(#[from] OxigraphEndpointError),

    /// reqwest-level HTTP error (used for the readiness probe).
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    /// URL parsing.
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    /// IRI parsing.
    #[error(transparent)]
    IriParse(#[from] IriParseError),

    /// IriS-level error.
    #[error(transparent)]
    IriS(#[from] IriSError),

    /// Result of `sparesults` parsing.
    #[error("SPARQL results parser: {0}")]
    SparResults(#[from] sparesults::QueryResultsParseError),

    /// The backend is read-only and a mutating operation was attempted.
    #[error("QLever backend is read-only: {operation}")]
    ReadOnly { operation: &'static str },

    /// A SPARQL solution did not contain an expected value.
    #[error("SPARQL solution missing {value}: {solution}")]
    NotFoundInSolution { value: String, solution: String },

    /// A SPARQL solution contained a term that was not an IRI where one was expected.
    #[error("SPARQL solution: expected IRI, got {value}")]
    SolutionNotIri { value: Term },

    /// A compressed input was supplied but no decompressor for that family
    /// was found on `$PATH`.
    #[error("no decompressor for {family} on PATH (tried {tried})")]
    DecompressorMissing { family: &'static str, tried: String },

    /// The host decompressor process exited with a non-zero status while
    /// streaming bytes into the index builder.
    #[error("decompressor {program} exited {status}\nstderr tail:\n{stderr_tail}")]
    DecompressorExit {
        program: String,
        status: i32,
        stderr_tail: String,
    },

    /// A compressed input has an inner extension QLever cannot index
    /// natively (e.g. `.jsonld.bz2`).
    #[error(
        "compressed input {path} has unsupported inner extension {suffix} (need .nt/.ttl/.nq before the compression suffix)"
    )]
    UnsupportedCompressedFormat { path: PathBuf, suffix: String },
}

impl QleverError {
    pub(crate) fn read_only(operation: &'static str) -> Self {
        QleverError::ReadOnly { operation }
    }
}
