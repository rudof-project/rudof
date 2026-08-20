use crate::cli::parser::PgschemaValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `pgschema-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute PgSchema Validate command logic.
pub struct PgschemaValidateCommand {
    /// Arguments specific to PgSchema Validate command.
    args: PgschemaValidateArgs,
}

impl PgschemaValidateCommand {
    pub fn new(args: PgschemaValidateArgs) -> Self {
        Self { args }
    }
}

impl Command for PgschemaValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "pgschema-validate"
    }

    /// Executes the PgSchema Validate command logic.
    ///
    /// With no `data`/`--schema`/`--typemap`, there is nothing new to load
    /// for that piece: this reuses whatever is already loaded in the
    /// session (useful in the interactive shell, where state persists
    /// across commands).
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let result_format = self.args.result_validation_format.into();

        if !self.args.data.is_empty() {
            ctx.rudof
                .load_data()
                .with_data(&self.args.data)
                .with_data_format(&data_format)
                .execute()?;
        }

        if let Some(schema) = &self.args.schema {
            ctx.rudof.load_pg_schema(schema).execute()?;
        }

        if let Some(typemap) = &self.args.typemap {
            ctx.rudof.load_typemap(typemap).execute()?;
        }

        ctx.rudof.validate_pgschema().execute()?;

        ctx.rudof
            .serialize_pgschema_validation_results(&mut ctx.writer)
            .with_result_pg_schema_validation_format(&result_format)
            .execute()?;

        Ok(())
    }
}
