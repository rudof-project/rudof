use clap::Args;
use std::path::PathBuf;

/// Arguments for the `connect` command
///
/// Opens (creating it if necessary) a LadybugDB database and stores the
/// connection details in a file so that stateless commands like `load` or
/// `query --cypher` can reuse them later (see discussion #748).
#[derive(Debug, Clone, Args)]
pub struct ConnectArgs {
    /// Path to the LadybugDB database directory
    /// Path to the LadybugDB database directory (not needed with --in-memory)
    #[arg(required_unless_present = "in_memory")]
    pub path: Option<PathBuf>,

    /// Create the database in memory (transient: connection details cannot be persisted)
    #[arg(long, short = 'm')]
    pub in_memory: bool,

    /// Open the database in read-only mode
    #[arg(long, short = 'r')]
    pub read_only: bool,

    /// File where connection details are stored (default: .rudof-connection.toml)
    #[arg(long, value_name = "FILE")]
    pub connection: Option<PathBuf>,
}
