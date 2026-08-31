use crate::{
    CypherQueryResult, PgDbInfo, Result, Rudof,
    api::pg_db::implementations::{connect_pg_db, load_pg_db, pg_db_ddl, query_cypher, reset_pg_db_connection},
    formats::{BackendSpec, DataFormat, DataReaderMode, DdlDialect, InputSpec, ShaclFormat},
};
use std::io;
use std::path::Path;

/// Operations for working with a property graph database (currently backed
/// by LadybugDB) and for deriving/emitting property graph DDL from RDF data.
pub trait PgDbOperations {
    /// Opens (creating if necessary) a property graph database and stores
    /// the connection info in `Rudof`'s state, so `load_pg_db`/`query_cypher`
    /// can reuse it without repeating the path.
    #[allow(clippy::too_many_arguments)]
    fn connect_pg_db(
        &mut self,
        path: Option<&Path>,
        in_memory: bool,
        read_only: bool,
        engine: Option<&BackendSpec>,
    ) -> Result<PgDbInfo>;

    /// Derives a property graph schema from `data` and emits it as DDL for
    /// `dialect`. Does not touch any loaded RDF data or database connection.
    #[allow(clippy::too_many_arguments)]
    fn pg_db_ddl(
        &self,
        data: &[InputSpec],
        dialect: Option<&DdlDialect>,
        graph_type_name: Option<&str>,
        data_format: Option<&DataFormat>,
        base_data: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<String>;

    /// Loads `data`, validates it against SHACL shapes (unless
    /// `skip_validation`), derives a property graph schema, and copies the
    /// data into the connected database.
    #[allow(clippy::too_many_arguments)]
    fn load_pg_db<W: io::Write>(
        &mut self,
        data: &[InputSpec],
        db_path: Option<&Path>,
        db_read_only: bool,
        shapes: Option<&InputSpec>,
        shapes_format: Option<&ShaclFormat>,
        base_shapes: Option<&str>,
        skip_validation: bool,
        data_format: Option<&DataFormat>,
        base_data: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
        writer: &mut W,
    ) -> Result<(usize, usize)>;

    /// Runs a Cypher query against the connected database. `query` is a
    /// file, a URL, `-` for stdin, or the query text itself.
    fn query_cypher(
        &mut self,
        query: &InputSpec,
        db_path: Option<&Path>,
        db_read_only: bool,
    ) -> Result<CypherQueryResult>;

    /// Clears the property graph database connection info.
    fn reset_pg_db_connection(&mut self);
}

impl PgDbOperations for Rudof {
    fn connect_pg_db(
        &mut self,
        path: Option<&Path>,
        in_memory: bool,
        read_only: bool,
        engine: Option<&BackendSpec>,
    ) -> Result<PgDbInfo> {
        connect_pg_db(self, path, in_memory, read_only, engine)
    }

    fn pg_db_ddl(
        &self,
        data: &[InputSpec],
        dialect: Option<&DdlDialect>,
        graph_type_name: Option<&str>,
        data_format: Option<&DataFormat>,
        base_data: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
    ) -> Result<String> {
        pg_db_ddl(
            self,
            data,
            dialect,
            graph_type_name,
            data_format,
            base_data,
            reader_mode,
        )
    }

    fn load_pg_db<W: io::Write>(
        &mut self,
        data: &[InputSpec],
        db_path: Option<&Path>,
        db_read_only: bool,
        shapes: Option<&InputSpec>,
        shapes_format: Option<&ShaclFormat>,
        base_shapes: Option<&str>,
        skip_validation: bool,
        data_format: Option<&DataFormat>,
        base_data: Option<&str>,
        reader_mode: Option<&DataReaderMode>,
        writer: &mut W,
    ) -> Result<(usize, usize)> {
        load_pg_db(
            self,
            data,
            db_path,
            db_read_only,
            shapes,
            shapes_format,
            base_shapes,
            skip_validation,
            data_format,
            base_data,
            reader_mode,
            writer,
        )
    }

    fn query_cypher(
        &mut self,
        query: &InputSpec,
        db_path: Option<&Path>,
        db_read_only: bool,
    ) -> Result<CypherQueryResult> {
        query_cypher(self, query, db_path, db_read_only)
    }

    fn reset_pg_db_connection(&mut self) {
        reset_pg_db_connection(self)
    }
}
