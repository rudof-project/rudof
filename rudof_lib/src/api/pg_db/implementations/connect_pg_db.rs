use crate::{PgDbConnection, PgDbInfo, Result, Rudof, errors::PgDbError, formats::BackendSpec};
use std::path::Path;

/// Opens (creating if necessary) a property graph database and, unless
/// `in_memory`, stores the connection info in `rudof.pg_db_connection` so
/// `load_pg_db`/`query_cypher` can reuse it without repeating the path.
///
/// In-memory databases cannot be reused by later calls since they do not
/// outlive the process, so their connection info is deliberately not stored.
pub fn connect_pg_db(
    rudof: &mut Rudof,
    path: Option<&Path>,
    in_memory: bool,
    read_only: bool,
    engine: Option<&BackendSpec>,
) -> Result<PgDbInfo> {
    let engine = engine.cloned().unwrap_or(BackendSpec::Lbug);
    if engine != BackendSpec::Lbug {
        return Err(PgDbError::UnsupportedEngine {
            engine: engine.to_string(),
        }
        .into());
    }

    if in_memory {
        super::db::verify_in_memory(read_only)?;
    } else {
        let path = path.ok_or_else(|| PgDbError::DataSourceSpec {
            message: "A database path is required unless connecting in-memory".to_string(),
        })?;
        let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
        let conn_info = PgDbConnection {
            engine,
            path: canonical,
            read_only,
        };
        super::db::verify_connection(&conn_info)?;
        rudof.pg_db_connection = Some(conn_info);
    }

    Ok(PgDbInfo {
        storage_version: lbug::get_storage_version(),
        library_source: lbug::get_library_source().to_string(),
    })
}
