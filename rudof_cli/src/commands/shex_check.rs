use crate::cli::parser::ShexCheckArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `shex-check` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to check whether a ShEx schema is
/// well-formed, without loading it into the session or requiring any RDF
/// data or ShapeMap.
pub struct ShexCheckCommand {
    /// Arguments specific to the ShexCheck command.
    args: ShexCheckArgs,
}

impl ShexCheckCommand {
    pub fn new(args: ShexCheckArgs) -> Self {
        Self { args }
    }
}

impl Command for ShexCheckCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shex-check"
    }

    /// Executes the ShexCheck command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let format = self.args.schema_format.into();

        let mut check_shex_schema = ctx.rudof.check_shex_schema(&self.args.schema, &mut ctx.writer);
        check_shex_schema = check_shex_schema.with_shex_schema_format(&format);
        if let Some(base) = &self.args.base {
            check_shex_schema = check_shex_schema.with_base(base);
        }
        check_shex_schema.execute()?;

        Ok(())
    }
}
