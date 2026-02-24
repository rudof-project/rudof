use crate::cli::parser::GenerateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Result, anyhow};
use rudof_lib::{Rudof, RudofConfig};

/// Implementation of the `generate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Generate command logic.
pub struct GenerateCommand {
    /// Arguments specific to Generate command.
    args: GenerateArgs,
}

impl GenerateCommand {
    pub fn new(args: GenerateArgs) -> Self {
        Self { args }
    }
}

impl Command for GenerateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "generate"
    }

    /// Executes the Generate command logic.
    fn execute(&self, _ctx: &mut CommandContext) -> Result<()> {
        // Create tokio runtime for async execution
        let runtime =
            tokio::runtime::Runtime::new().map_err(|e| anyhow!("Failed to create tokio runtime: {e}"))?;

        runtime.block_on(async {
            // Create a temporary Rudof instance for generation (doesn't need existing state)
            let rudof = Rudof::new(&RudofConfig::default_config()?)?;

            rudof
                .generate_data(
                    &self.args.schema,
                    &(&self.args.schema_format).into(),
                    self.args.entity_count,
                    &self.args.common.output,
                    &(&self.args.result_format).into(),
                    self.args.seed,
                    self.args.parallel,
                    &self.args.common.config,
                )
                .await
        })?;

        Ok(())
    }
}
