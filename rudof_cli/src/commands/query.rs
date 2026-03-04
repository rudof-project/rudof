use crate::cli::parser::QueryArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::{ReaderMode, rdf_reader_mode::RDFReaderMode};

/// Implementation of the `query` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Query command logic.
pub struct QueryCommand {
    /// Arguments specific to Query command.
    args: QueryArgs,
}

impl QueryCommand {
    pub fn new(args: QueryArgs) -> Self {
        Self { args }
    }
}

impl Command for QueryCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "query"
    }

    /// Executes the Query command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();

        ctx.rudof.load_data(
            &self.args.data,
            &(&self.args.data_format).into(),
            &self.args.base,
            &self.args.endpoint,
            &reader_mode,
            false,
        )?;

        ctx.rudof.execute_query(
            &self.args.query,
            &(&self.args.query_type).into(),
            &(&self.args.result_query_format).into(),
            &mut ctx.writer,
        )?;

        Ok(())
    }
}
