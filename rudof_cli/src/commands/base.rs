use crate::cli::parser::{
    Command as CliCommand, CommonArgs, CommonArgsAll, CommonArgsOutputForceOverWrite,
};
use crate::commands::{
    McpCommand, ShapemapCommand, ShexCommand, PgschemaCommand, ValidateCommand,
};
use crate::output::{ColorSupport, get_writer};
use anyhow::Result;
use rudof_lib::RudofConfig;
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

    /// Performs pre-execution validation of specific command arguments.
    /// 
    /// This is called after CLI parsing but before [Self::execute].
    fn validate(&self) -> Result<()> {
        Ok(())
    }
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

    /// Configuration (from rudof_lib)
    pub config: RudofConfig,

    /// Debug level
    pub debug_level: u8,

    /// Force overwrite flag
    pub force_overwrite: bool,

    /// Color support
    pub color: ColorSupport,
}

impl CommandContext {
    pub fn new(
        writer: Box<dyn Write>,
        config: RudofConfig,
        debug_level: u8,
        force_overwrite: bool,
        color: ColorSupport,
    ) -> Self {
        Self {
            writer,
            config,
            debug_level,
            force_overwrite,
            color,
        }
    }

    /// Initializes a [CommandContext] from the parsed [CliCommand].
    /// 
    /// This method handles loading the configuration file and 
    /// initializing the output writer based on CLI flags.
    pub fn from_cli(cmd: &CliCommand, debug: u8) -> Result<Self> {
        let common = extract_common(cmd);

        // Load config
        let config = match &common.config() {
            Some(path) => RudofConfig::from_path(path)?,
            None => RudofConfig::default_config()?,
        };

        // Determine the appropriate writer and detect color support
        let (writer, color) = get_writer(&common.output().cloned(), common.force_overwrite())?;

        Ok(Self {
            writer,
            config,
            debug_level: debug,
            force_overwrite: common.force_overwrite(),
            color,
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
        }
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper function to extract [CommonArgs] from any [CliCommand] variant.
fn extract_common(cmd: &CliCommand) -> CommonArgs {
    match cmd {
        CliCommand::Mcp(_) => CommonArgs::None,
        CliCommand::Shapemap(a) => CommonArgs::OutputForceOverWrite(CommonArgsOutputForceOverWrite {
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Shex(a) => CommonArgs::All(CommonArgsAll { 
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Pgschema(a) => CommonArgs::All(CommonArgsAll { 
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
        CliCommand::Validate(a) => CommonArgs::All(CommonArgsAll { 
            config: a.common.config.clone(),
            output: a.common.output.clone(),
            force_overwrite: a.common.force_overwrite,
        }),
    }
}