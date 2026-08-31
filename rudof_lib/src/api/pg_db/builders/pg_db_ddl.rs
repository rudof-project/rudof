use crate::{
    Result, Rudof,
    api::pg_db::PgDbOperations,
    formats::{DataFormat, DataReaderMode, DdlDialect, InputSpec},
};

/// Builder for the `pg_db_ddl` operation.
///
/// Takes `&Rudof` (not `&mut`): deriving/emitting DDL never touches loaded
/// RDF data or database connection state, matching how the CLI's `ddl`
/// command never opens a database.
pub struct PgDbDdlBuilder<'a> {
    rudof: &'a Rudof,
    data: &'a [InputSpec],
    dialect: Option<&'a DdlDialect>,
    graph_type_name: Option<&'a str>,
    data_format: Option<&'a DataFormat>,
    base_data: Option<&'a str>,
    reader_mode: Option<&'a DataReaderMode>,
}

impl<'a> PgDbDdlBuilder<'a> {
    /// Creates a new builder.
    ///
    /// Internal helper called by `Rudof::pg_db_ddl()`; not intended for
    /// public construction by callers.
    pub(crate) fn new(rudof: &'a Rudof, data: &'a [InputSpec]) -> Self {
        Self {
            rudof,
            data,
            dialect: None,
            graph_type_name: None,
            data_format: None,
            base_data: None,
            reader_mode: None,
        }
    }

    /// Set the DDL dialect to generate (default: `DdlDialect::Cypher`).
    pub fn with_dialect(mut self, dialect: &'a DdlDialect) -> Self {
        self.dialect = Some(dialect);
        self
    }

    /// Set the graph type name used by the `gql` dialect.
    pub fn with_graph_type_name(mut self, graph_type_name: &'a str) -> Self {
        self.graph_type_name = Some(graph_type_name);
        self
    }

    /// Set the RDF data format.
    pub fn with_data_format(mut self, data_format: &'a DataFormat) -> Self {
        self.data_format = Some(data_format);
        self
    }

    /// Set the base IRI for the data.
    pub fn with_base_data(mut self, base_data: &'a str) -> Self {
        self.base_data = Some(base_data);
        self
    }

    /// Set the RDF reader mode.
    pub fn with_reader_mode(mut self, reader_mode: &'a DataReaderMode) -> Self {
        self.reader_mode = Some(reader_mode);
        self
    }

    /// Execute the `pg_db_ddl` operation with the configured parameters.
    pub fn execute(self) -> Result<String> {
        <Rudof as PgDbOperations>::pg_db_ddl(
            self.rudof,
            self.data,
            self.dialect,
            self.graph_type_name,
            self.data_format,
            self.base_data,
            self.reader_mode,
        )
    }
}
