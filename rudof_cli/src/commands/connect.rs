use crate::cli::parser::ConnectArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Context, Result, anyhow};
use lbug::{Connection, Database, SystemConfig};
use std::path::{Path, PathBuf};

/// File where `rudof connect` stores connection details by default.
pub const DEFAULT_CONNECTION_FILE: &str = ".rudof-connection.toml";

/// Connection details persisted by `rudof connect` and consumed by the
/// stateless `load` and `query --dialect cypher` commands (see discussions
/// #747 and #748: the CLI stays stateless — commands remain deterministic —
/// while the connection itself is reused through this file).
#[derive(Debug, Clone)]
pub struct ConnectionDetails {
    /// Database engine. Only `lbug` (LadybugDB) is currently supported; see
    /// [`crate::cli::wrappers::DbEngineCli`] for the CLI-facing enum this
    /// mirrors.
    pub engine: String,
    /// Path to the database directory.
    pub path: PathBuf,
    /// Whether the database was opened in read-only mode.
    pub read_only: bool,
}

impl ConnectionDetails {
    pub fn to_toml_string(&self) -> Result<String> {
        let mut map = toml::map::Map::new();
        map.insert("engine".to_string(), toml::Value::String(self.engine.clone()));
        map.insert("path".to_string(), toml::Value::String(self.path.display().to_string()));
        map.insert("read_only".to_string(), toml::Value::Boolean(self.read_only));
        toml::to_string_pretty(&toml::Value::Table(map)).context("Failed to serialize connection details")
    }

    pub fn from_toml_str(s: &str) -> Result<Self> {
        let value: toml::Value = toml::from_str(s).context("Failed to parse connection details file")?;
        let engine = value
            .get("engine")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Connection details file is missing 'engine'"))?
            .to_string();
        if engine != "lbug" {
            return Err(anyhow!("Unsupported database engine '{engine}' in connection details"));
        }
        let path = PathBuf::from(
            value
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Connection details file is missing 'path'"))?,
        );
        let read_only = value.get("read_only").and_then(|v| v.as_bool()).unwrap_or(false);
        Ok(Self {
            engine,
            path,
            read_only,
        })
    }

    pub fn write_to_file(&self, file: &Path) -> Result<()> {
        std::fs::write(file, self.to_toml_string()?)
            .with_context(|| format!("Failed to write connection details to '{}'", file.display()))?;
        Ok(())
    }

    /// Load connection details from `file`, or from [`DEFAULT_CONNECTION_FILE`]
    /// in the current directory when `file` is `None`.
    ///
    /// Returns `None` when no connection details file can be found, so that
    /// callers can report a helpful error.
    pub fn load(file: Option<&Path>) -> Result<Option<Self>> {
        let path = file
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_CONNECTION_FILE));
        if !path.exists() {
            return Ok(None);
        }
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read connection details from '{}'", path.display()))?;
        Self::from_toml_str(&contents)
            .map(Some)
            .with_context(|| format!("Invalid connection details in '{}'", path.display()))
    }

    /// Resolve the database to operate on: an explicit `--db` path wins,
    /// otherwise the connection details file is consulted.
    pub fn resolve(db: Option<&Path>, connection_file: Option<&Path>) -> Result<Self> {
        if let Some(db_path) = db {
            return Ok(Self {
                engine: "lbug".to_string(),
                path: db_path.to_path_buf(),
                read_only: false,
            });
        }
        match Self::load(connection_file)? {
            Some(details) => Ok(details),
            None => Err(anyhow!(
                "No database specified. Run `rudof connect <db>` first, pass --db <path>, \
                 or provide a connection details file with --connection <file>"
            )),
        }
    }
}

/// Implementation of the `connect` command.
pub struct ConnectCommand {
    args: ConnectArgs,
}

impl ConnectCommand {
    pub fn new(args: ConnectArgs) -> Self {
        Self { args }
    }
}

impl Command for ConnectCommand {
    fn name(&self) -> &'static str {
        "connect"
    }

    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let args = &self.args;
        let db_path = args.path.as_deref();

        let db = if args.in_memory {
            Database::in_memory(SystemConfig::default())
        } else {
            Database::new(
                db_path.expect("path is required without --in-memory"),
                SystemConfig::default().read_only(args.read_only),
            )
        }
        .context("Failed to create/open LadybugDB database")?;

        let _conn = Connection::new(&db).context("Failed to connect to LadybugDB")?;
        std::mem::drop(_conn);

        writeln!(ctx.writer, "LadybugDB database opened successfully")?;
        if args.in_memory {
            writeln!(ctx.writer, "  Mode: in-memory")?;
        } else {
            let path = db_path.expect("path is required without --in-memory");
            writeln!(ctx.writer, "  Path: {}", path.display())?;
            if args.read_only {
                writeln!(ctx.writer, "  Read-only: true")?;
            }
        }
        writeln!(ctx.writer, "  Builder storage version: {}", lbug::get_storage_version())?;
        writeln!(ctx.writer, "  Library source: {}", lbug::get_library_source())?;

        if args.in_memory {
            writeln!(
                ctx.writer,
                "  Note: connection details were not stored because in-memory databases \
                 do not outlive this process"
            )?;
        } else {
            let path = db_path.expect("path is required without --in-memory");
            let canonical = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
            let details = ConnectionDetails {
                engine: args.engine.to_string(),
                path: canonical,
                read_only: args.read_only,
            };
            let file = args
                .connection
                .clone()
                .unwrap_or_else(|| PathBuf::from(DEFAULT_CONNECTION_FILE));
            details.write_to_file(&file)?;
            writeln!(
                ctx.writer,
                "  Connection details stored in '{}' (used by `load` and `query --dialect cypher`)",
                file.display()
            )?;
        }

        let _ = db;
        Ok(())
    }
}
