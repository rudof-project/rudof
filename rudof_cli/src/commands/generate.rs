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
        let runtime = tokio::runtime::Runtime::new().map_err(|e| anyhow!("Failed to create tokio runtime: {e}"))?;

        runtime.block_on(async {
            // Create a temporary Rudof instance for generation (doesn't need existing state)
            let rudof = Rudof::new(RudofConfig::default());

            let schema_format = self.args.schema_format.into();
            let result_format = self.args.result_format.into();

            // `-n`/`--entities` always wins when given. Otherwise, a `--generator-config`
            // file's own `entity_count` is left in effect (`None`); with no config file
            // there is nothing else to fall back on, so keep the documented default of 10.
            let entity_count = match (self.args.entity_count, &self.args.generator_config) {
                (Some(n), _) => Some(n),
                (None, Some(_)) => None,
                (None, None) => Some(10),
            };

            let mut generation = rudof
                .generate_data(&self.args.schema, &schema_format, entity_count)
                .with_result_generation_format(&result_format);

            if let Some(seed) = self.args.seed {
                generation = generation.with_seed(seed);
            }
            if let Some(parallel) = self.args.parallel {
                generation = generation.with_parallel(parallel);
            }
            if let Some(output) = &self.args.common.output {
                generation = generation.with_output(output);
            }
            if let Some(generator_config) = &self.args.generator_config {
                generation = generation.with_config_file(generator_config);
            }

            generation.execute().await
        })?;

        Ok(())
    }
}
