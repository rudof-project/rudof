use crate::cli::parser::{
    Command as CliCommand, CommonArgs, CommonArgsAll, CommonArgsNoBackend, CommonArgsOutputForceOverWrite,
};
use crate::commands::{
    CompareCommand, CompletionCommand, ConfigCommand, ConvertCommand, DataCommand, DctapCommand, GenerateCommand,
    MaterializeCommand, McpCommand, NodeCommand, PgschemaCommand, PgschemaValidateCommand, QueryCommand,
    RdfConfigCommand, ServiceCommand, ShaclCommand, ShaclValidateCommand, ShapemapCommand, ShexCommand,
    ShexValidateCommand, SparqlCommand, ValidateCommand,
};
use crate::output::{ColorSupport, get_writer};
use crate::shell::ShellCommand;
use anyhow::Result;
use rudof_lib::{Rudof, RudofConfig};
use std::io::Write;

// ============================================================================
// Command Trait
// ============================================================================

/// The core command trait that all commands must implement
pub trait Command: Send + Sync {
    /// Executes the command's logic using the provided [CommandContext].
    fn execute(&self, ctx: &mut CommandContext) -> Result<()>;

    /// Returns a static string identifying the command.
    ///
    /// Useful for logging, telemetry, and debugging.
    fn name(&self) -> &'static str;
}

// ============================================================================
// Command Context
// ============================================================================

/// The shared environment and state required for command execution.
///
/// This structure bundles output handles, global configuration,
/// and UI preferences (like color and verbosity).
pub struct CommandContext {
    /// Output writer (stdout, file, etc.)
    pub writer: Box<dyn Write>,

    /// Rudof (from rudof_lib)
    pub rudof: Rudof,

    /// Debug level
    pub debug_level: u8,

    /// Color support
    pub color: ColorSupport,

    /// Name of the SPARQL endpoint activated via the shell's `endpoint`
    /// command, if any. Only ever set by [`crate::shell`]; unused outside
    /// the interactive shell.
    pub active_endpoint: Option<String>,
}

impl CommandContext {
    pub fn new(writer: Box<dyn Write>, rudof: Rudof, debug_level: u8, color: ColorSupport) -> Self {
        Self {
            writer,
            rudof,
            debug_level,
            color,
            active_endpoint: None,
        }
    }

    /// Initializes a [CommandContext] from the parsed [CliCommand].
    ///
    /// This method handles loading the configuration file and
    /// initializing the output writer based on CLI flags.
    pub fn from_cli(cmd: &CliCommand, debug: u8) -> Result<Self> {
        let common = extract_common(cmd);

        // Load config
        let config = RudofConfig::discover(common.config().map(|p| p.as_path()))?;

        // A persisted `logging.level` (e.g. from `~/.config/rudof/config.toml`
        // or a discovered `rudof.toml`) sets the startup tracing filter, same
        // as `$RUST_LOG` — but `$RUST_LOG`, when the user has explicitly set
        // it, wins, matching every other tool that honours it.
        if std::env::var_os("RUST_LOG").is_none()
            && let Some(level) = config.logging().level()
        {
            crate::logging::set_level(level)
                .map_err(|err| anyhow::anyhow!("invalid 'logging.level' value '{level}' in config: {err}"))?;
        }

        // Initialize Rudof with the loaded configuration
        let rudof = Rudof::new(config);

        // Determine the appropriate writer and detect color support
        let (writer, color) = get_writer(&common.output().cloned(), common.force_overwrite())?;

        Ok(Self {
            writer,
            rudof,
            debug_level: debug,
            color,
            active_endpoint: None,
        })
    }

    /// Returns true if the output supports and is configured for ANSI colors.
    pub fn use_color(&self) -> bool {
        self.color.enabled()
    }
}

// ============================================================================
// Command Factory
// ============================================================================

/// Responsible for instantiating [Command] implementations based on CLI input.
pub struct CommandFactory;

impl CommandFactory {
    /// Maps a [CliCommand] enum variant to its corresponding [Command] trait object.
    pub fn create(cli_command: CliCommand) -> Result<Box<dyn Command>> {
        match cli_command {
            CliCommand::Mcp(args) => Ok(Box::new(McpCommand::new(args))),
            CliCommand::Shapemap(args) => Ok(Box::new(ShapemapCommand::new(args))),
            CliCommand::Shex(args) => Ok(Box::new(ShexCommand::new(args))),
            CliCommand::Pgschema(args) => Ok(Box::new(PgschemaCommand::new(args))),
            CliCommand::Validate(args) => Ok(Box::new(ValidateCommand::new(args))),
            CliCommand::ShexValidate(args) => Ok(Box::new(ShexValidateCommand::new(args))),
            CliCommand::ShaclValidate(args) => Ok(Box::new(ShaclValidateCommand::new(args))),
            CliCommand::Data(args) => Ok(Box::new(DataCommand::new(args))),
            CliCommand::Node(args) => Ok(Box::new(NodeCommand::new(args))),
            CliCommand::Shacl(args) => Ok(Box::new(ShaclCommand::new(args))),
            CliCommand::DCTap(args) => Ok(Box::new(DctapCommand::new(args))),
            CliCommand::Convert(args) => Ok(Box::new(ConvertCommand::new(args))),
            CliCommand::Compare(args) => Ok(Box::new(CompareCommand::new(args))),
            CliCommand::RdfConfig(args) => Ok(Box::new(RdfConfigCommand::new(args))),
            CliCommand::Service(args) => Ok(Box::new(ServiceCommand::new(args))),
            CliCommand::Query(args) => Ok(Box::new(QueryCommand::new(args))),
            CliCommand::Sparql(args) => Ok(Box::new(SparqlCommand::new(args))),
            CliCommand::Generate(args) => Ok(Box::new(GenerateCommand::new(args))),
            CliCommand::Materialize(args) => Ok(Box::new(MaterializeCommand::new(args))),
            CliCommand::PgschemaValidate(args) => Ok(Box::new(PgschemaValidateCommand::new(args))),
            CliCommand::Completion(args) => Ok(Box::new(CompletionCommand::new(args))),
            CliCommand::Config(args) => Ok(Box::new(ConfigCommand::new(args))),
            CliCommand::Shell(args) => Ok(Box::new(ShellCommand::new(args))),
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper function to extract [CommonArgs] from any [CliCommand] variant.
pub(crate) fn extract_common(cmd: &CliCommand) -> CommonArgs {
    match cmd {
        CliCommand::Mcp(_) => CommonArgs::None,
        CliCommand::Shapemap(a) => CommonArgs::OutputForceOverWrite(CommonArgsOutputForceOverWrite {
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Shex(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Pgschema(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Validate(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::ShexValidate(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::ShaclValidate(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::Data(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::Node(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::Shacl(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::DCTap(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Convert(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Compare(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::RdfConfig(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Service(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Query(a) => CommonArgs::All(CommonArgsAll {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
            backend: a.common.backend.clone(),
            endpoint: a.common.endpoint.clone(),
        }),
        CliCommand::Sparql(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Generate(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Materialize(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::PgschemaValidate(a) => CommonArgs::OutputForceOverWrite(CommonArgsOutputForceOverWrite {
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Completion(a) => CommonArgs::OutputForceOverWrite(CommonArgsOutputForceOverWrite {
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Config(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Shell(a) => CommonArgs::NoBackend(CommonArgsNoBackend {
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
    }
}
