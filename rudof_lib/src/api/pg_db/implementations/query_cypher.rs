use crate::{CypherQueryResult, Result, Rudof, errors::PgDbError, formats::InputSpec};
use std::io::Read;
use std::path::Path;

/// Runs a Cypher query against the connected database: an explicit
/// `db_path` override wins, otherwise the connection info stored by a prior
/// `connect_pg_db` call is used.
pub fn query_cypher(
    rudof: &mut Rudof,
    query: &InputSpec,
    db_path: Option<&Path>,
    db_read_only: bool,
) -> Result<CypherQueryResult> {
    let conn_info = super::resolve_connection(rudof, db_path, db_read_only)?;

    let mut reader = query.open_read(None, "Cypher query").map_err(|error| PgDbError::DataSourceSpec {
        message: format!("Failed to open Cypher query from '{}': {error}", query.source_name()),
    })?;
    let mut cypher = String::new();
    reader.read_to_string(&mut cypher).map_err(|error| PgDbError::DataSourceSpec {
        message: format!("Failed to read Cypher query from '{}': {error}", query.source_name()),
    })?;

    let result = super::db::run_cypher_query(&conn_info, cypher.trim())?;
    Ok(result)
}
