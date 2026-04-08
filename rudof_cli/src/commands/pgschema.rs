use crate::cli::parser::PgschemaArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `pgschema` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Property Graph Schema logic.
pub struct PgschemaCommand {
    /// Arguments specific to Property Graph Schema command.
    args: PgschemaArgs,
}

impl PgschemaCommand {
    pub fn new(args: PgschemaArgs) -> Self {
        Self { args }
    }
}

impl Command for PgschemaCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "pgschema"
    }

    /// Executes the Property Graph Schema command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let pg_schema_format = self.args.schema_format.into();
        let result_pg_schema_format = self.args.result_schema_format.into();

        ctx.rudof
            .load_pg_schema(&self.args.schema)
            .with_pg_schema_format(&pg_schema_format)
            .execute()?;

        ctx.rudof
            .serialize_pg_schema(&mut ctx.writer)
            .with_result_pg_schema_format(&result_pg_schema_format)
            .execute()?;

        Ok(())
    }
}
