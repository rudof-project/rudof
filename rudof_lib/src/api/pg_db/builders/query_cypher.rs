use crate::{CypherQueryResult, Result, Rudof, api::pg_db::PgDbOperations, formats::InputSpec};
use std::path::Path;

/// Builder for the `query_cypher` operation.
pub struct QueryCypherBuilder<'a> {
    rudof: &'a mut Rudof,
    query: &'a InputSpec,
    db_path: Option<&'a Path>,
    db_read_only: bool,
}

impl<'a> QueryCypherBuilder<'a> {
    /// Creates a new builder.
    ///
    /// Internal helper called by `Rudof::query_cypher()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a mut Rudof, query: &'a InputSpec) -> Self {
        Self {
            rudof,
            query,
            db_path: None,
            db_read_only: false,
        }
    }

    /// Override the database to query (otherwise the connection info stored
    /// by a prior `connect_pg_db` call is used).
    pub fn with_db(mut self, path: &'a Path, read_only: bool) -> Self {
        self.db_path = Some(path);
        self.db_read_only = read_only;
        self
    }

    /// Execute the `query_cypher` operation with the configured parameters.
    pub fn execute(self) -> Result<CypherQueryResult> {
        <Rudof as PgDbOperations>::query_cypher(self.rudof, self.query, self.db_path, self.db_read_only)
    }
}
