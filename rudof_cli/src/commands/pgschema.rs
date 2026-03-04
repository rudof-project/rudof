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
        let mut reader = self.args.schema.open_read(None, "PGSchema")?;

        let schema = ctx.rudof.get_schema(&mut reader)?;

        write!(ctx.writer, "{schema}")?;

        Ok(())
    }
}
