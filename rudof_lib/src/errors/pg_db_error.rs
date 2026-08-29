use thiserror::Error;

/// Errors that can occur when working with a property graph database
/// (currently backed by LadybugDB) or deriving/emitting property graph DDL
/// from RDF data.
#[derive(Error, Debug)]
pub enum PgDbError {
    /// The database engine specified is not supported by Rudof.
    #[error("Unsupported database engine '{engine}'. Valid engines are: lbug")]
    UnsupportedEngine { engine: String },

    /// The DDL dialect specified is not supported by Rudof.
    #[error("Unsupported DDL dialect '{dialect}'. Valid dialects are: cypher, gql")]
    UnsupportedDialect { dialect: String },

    /// Errors related to specifying the RDF data source.
    #[error("Data source specification error: {message}")]
    DataSourceSpec { message: String },

    /// Failed to open or create the database.
    #[error("Failed to open database at '{path}': {error}")]
    FailedOpenDatabase { path: String, error: String },

    /// Failed to open a connection to an already-open database.
    #[error("Failed to connect to database: {error}")]
    FailedConnect { error: String },

    /// No database connection available: neither an explicit path/connection
    /// file override nor a prior `connect` call in this session.
    #[error(
        "No database specified. Call `connect_pg_db` first, or pass an explicit database path."
    )]
    NoConnection,

    /// The database connection in use is read-only but a write was attempted.
    #[error("The database connection in use is read-only; a writable database is required")]
    ReadOnlyConnection,

    /// Failed to apply the derived DDL (create node/relationship tables) to the database.
    #[error("Failed to create table '{table}': {error}")]
    FailedCreateTable { table: String, error: String },

    /// Failed to insert a node into the database.
    #[error("Failed to insert node into '{table}': {error}")]
    FailedInsertNode { table: String, error: String },

    /// Failed to run a Cypher query.
    #[error("Cypher query failed: {error}")]
    FailedCypherQuery { error: String },

    /// No RDF data available to derive a property graph schema from.
    #[error("No RDF data loaded to derive a property graph schema from")]
    NoDataLoaded,

    /// SHACL validation failed (the data does not conform to the shapes).
    #[error("Data does not conform to SHACL shapes; aborting load ({violations} violation(s))")]
    ShaclViolations { violations: usize },

    /// Failed I/O operation while writing progress output.
    #[error("Failed I/O operation: {error}")]
    FailedIoOperation { error: String },
}
