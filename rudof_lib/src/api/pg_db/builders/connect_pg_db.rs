use crate::{PgDbInfo, Result, Rudof, api::pg_db::PgDbOperations, formats::DbEngine};
use std::path::Path;

/// Builder for the `connect_pg_db` operation.
pub struct ConnectPgDbBuilder<'a> {
    rudof: &'a mut Rudof,
    path: Option<&'a Path>,
    in_memory: bool,
    read_only: bool,
    engine: Option<&'a DbEngine>,
}

impl<'a> ConnectPgDbBuilder<'a> {
    /// Creates a new builder.
    ///
    /// Internal helper called by `Rudof::connect_pg_db()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a mut Rudof, path: Option<&'a Path>) -> Self {
        Self {
            rudof,
            path,
            in_memory: false,
            read_only: false,
            engine: None,
        }
    }

    /// Create the database in memory (transient: the connection info is not
    /// stored in `Rudof`'s state since it cannot outlive the process).
    pub fn with_in_memory(mut self, in_memory: bool) -> Self {
        self.in_memory = in_memory;
        self
    }

    /// Open the database in read-only mode.
    pub fn with_read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Set the database engine (default: `DbEngine::Lbug`, the only one
    /// supported today).
    pub fn with_engine(mut self, engine: &'a DbEngine) -> Self {
        self.engine = Some(engine);
        self
    }

    /// Execute the `connect_pg_db` operation with the configured parameters.
    pub fn execute(self) -> Result<PgDbInfo> {
        <Rudof as PgDbOperations>::connect_pg_db(self.rudof, self.path, self.in_memory, self.read_only, self.engine)
    }
}
