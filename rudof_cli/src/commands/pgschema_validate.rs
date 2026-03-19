use crate::cli::parser::PgSchemaValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `pgschema-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute PgSchema Validate command logic.
pub struct PgSchemaValidateCommand {
    /// Arguments specific to PgSchema Validate command.
    args: PgSchemaValidateArgs,
}

impl PgSchemaValidateCommand {
    pub fn new(args: PgSchemaValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for PgSchemaValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "pgschema-validate"
    }

    /// Executes the PgSchema Validate command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let shapemap_format = self.args.shapemap_format.into();
        let result_format = self.args.result_validation_format.into();

        ctx.rudof.load_data(&self.args.data).with_data_format(&data_format).execute()?;

        ctx.rudof.load_pg_schema(&self.args.schema).execute()?;

        ctx.rudof.load_shapemap(&self.args.shapemap).with_shapemap_format(&shapemap_format).execute()?;

        ctx.rudof.validate_pgschema().execute()?;

        ctx.rudof.serialize_pgschema_validation_results(&mut ctx.writer).with_result_pg_schema_validation_format(&result_format).execute()?;

        Ok(())
    }
}
