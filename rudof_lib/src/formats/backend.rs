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
    /// LadybugDB (see <https://github.com/LadybugDB/ladybug>), a local
    /// embedded property graph database. Only [`crate::Rudof::connect_pg_db`]
    /// and its related `ddl`/`load_pg_db`/`query_cypher` operations can
    /// actually connect to it today; selecting it for an RDF-loading
    /// operation (via [`LoadDataBuilder`](crate::api::data::builders::LoadDataBuilder))
    /// currently fails, since rudof can derive a property graph from RDF
    /// (`load`/`ddl`) but cannot yet read one back out as RDF triples — see
    /// <https://github.com/rudof-project/rudof/discussions/747>.
    Lbug,
}

impl Display for BackendSpec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BackendSpec::Memory => write!(f, "memory"),
            BackendSpec::Endpoint(s) => write!(f, "endpoint={s}"),
            BackendSpec::Qlever => write!(f, "qlever"),
            BackendSpec::Lbug => write!(f, "lbug"),
        }
    }
}

impl BackendSpec {
    /// `true` when the QLever backend was requested.
    pub fn is_qlever(&self) -> bool {
        matches!(self, BackendSpec::Qlever)
    }

    /// `true` when the LadybugDB backend was requested.
    pub fn is_lbug(&self) -> bool {
        matches!(self, BackendSpec::Lbug)
    }

    /// `Some(url_or_name)` when an endpoint was requested.
    pub fn endpoint(&self) -> Option<&str> {
        match self {
            BackendSpec::Endpoint(s) => Some(s.as_str()),
            _ => None,
        }
    }
}
