use std::fmt::{Display, Formatter};

/// Choice of RDF data backend for [`LoadDataBuilder`](crate::api::data::builders::LoadDataBuilder).
///
/// One source of truth shared between the lib and the CLI: every subcommand
/// that loads RDF data converts its `--backend` flag into a `BackendSpec`
/// before handing it to the builder.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum BackendSpec {
    /// Default: parse RDF into an in-process `OxigraphInMemory` graph.
    #[default]
    Memory,
    /// Send queries to a remote SPARQL endpoint (URL or a config-registered name).
    Endpoint(String),
    /// Launch a local QLever Docker container and index the input on disk.
    /// Requires the `qlever` feature on the workspace.
    Qlever,
}

impl Display for BackendSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendSpec::Memory => write!(f, "memory"),
            BackendSpec::Endpoint(s) => write!(f, "endpoint={s}"),
            BackendSpec::Qlever => write!(f, "qlever"),
        }
    }
}

impl BackendSpec {
    /// `true` when the QLever backend was requested.
    pub fn is_qlever(&self) -> bool {
        matches!(self, BackendSpec::Qlever)
    }

    /// `Some(url_or_name)` when an endpoint was requested.
    pub fn endpoint(&self) -> Option<&str> {
        match self {
            BackendSpec::Endpoint(s) => Some(s.as_str()),
            _ => None,
        }
    }
}
