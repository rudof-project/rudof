use crate::cli::parser::ConfigArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::TomlConfig;
use std::io::Write;

/// Implementation of the `config` command.
///
/// Dumps the effective [`RudofConfig`](rudof_lib::RudofConfig) that rudof resolved
/// from all configuration sources as TOML
pub struct ConfigCommand {
    #[allow(dead_code)]
    args: ConfigArgs,
}

impl ConfigCommand {
    pub fn new(args: ConfigArgs) -> Self {
        Self { args }
    }
}

impl Command for ConfigCommand {
    /// Executes the config command logic
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let toml = ctx.rudof.config().execute().to_toml_string()?;
        writeln!(ctx.writer, "{toml}")?;
        Ok(())
    }

    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "config"
    }
}
