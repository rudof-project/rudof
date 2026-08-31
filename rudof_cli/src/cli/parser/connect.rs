use crate::cli::wrappers::BackendKindCli;
use clap::Args;
use std::path::PathBuf;

/// Arguments for the `connect` command
///
/// Opens (creating it if necessary) a database and stores the connection
/// details in a file so that stateless commands like `load` or
/// `query --dialect cypher` can reuse them later (see discussion #748).
#[derive(Debug, Clone, Args)]
pub struct ConnectArgs {
    /// Path to the database directory
    /// Path to the database directory (not needed with --in-memory)
    #[arg(required_unless_present = "in_memory")]
    pub path: Option<PathBuf>,

    /// Backend to connect to. Uses the same `--backend` flag/type as
    /// `data`/`query`/`shacl-validate`/etc. (`memory | qlever | lbug |
    /// endpoint=<URL_OR_NAME>`), so a future backend needs no new flag here
    /// -- only `lbug` (LadybugDB) can actually be connected to today; any
    /// other value is rejected with a clear error.
    #[arg(
        long = "backend",
        value_name = "BACKEND",
        ignore_case = true,
        help = "Backend to connect to (memory | qlever | lbug | endpoint=<URL_OR_NAME>). \
                Only lbug can actually be connected to today; other values are rejected \
                with a clear error.",
        value_parser = clap::builder::ValueParser::new(|s: &str| {
            use std::str::FromStr;
            BackendKindCli::from_str(s)
        }),
        default_value_t = BackendKindCli::Lbug,
    )]
    pub backend: BackendKindCli,

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
