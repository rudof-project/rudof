mod connect_pg_db;
mod db;
mod load_pg_db;
mod pg_db_ddl;
mod query_cypher;
mod reset_pg_db_connection;
mod schema;

pub(crate) use connect_pg_db::connect_pg_db;
pub(crate) use load_pg_db::load_pg_db;
pub(crate) use pg_db_ddl::pg_db_ddl;
pub(crate) use query_cypher::query_cypher;
pub(crate) use reset_pg_db_connection::reset_pg_db_connection;

use crate::{PgDbConnection, Result, Rudof, errors::PgDbError, formats::BackendSpec};
use std::path::Path;

/// Resolves the connection to operate on for `load_pg_db`/`query_cypher`: an
/// explicit `db_path` override wins, otherwise the connection info stored by
/// a prior `connect_pg_db` call is used.
fn resolve_connection(rudof: &Rudof, db_path: Option<&Path>, db_read_only: bool) -> Result<PgDbConnection> {
    if let Some(path) = db_path {
        let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        return Ok(PgDbConnection {
            engine: BackendSpec::Lbug,
            path: canonical,
            read_only: db_read_only,
        });
    }
    let conn_info = rudof.pg_db_connection.clone().ok_or(PgDbError::NoConnection)?;
    Ok(conn_info)
}
