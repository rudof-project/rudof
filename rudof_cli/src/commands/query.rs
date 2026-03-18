use crate::cli::parser::QueryArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

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
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let query_type = self.args.query_type.into();

        let mut loading = ctx.rudof.load_data(&self.args.data).with_data_format(&data_format).with_reader_mode(&reader_mode);
        if let Some(base) = self.args.base.as_deref() { loading = loading.with_base(base); }
        if let Some(endpoint) = self.args.endpoint.as_deref() { loading = loading.with_endpoint(endpoint); }
        loading.execute()?;

        ctx.rudof.load_query(&self.args.query)
            .with_query_type(&query_type)
            .execute()?;

        ctx.rudof.run_query().execute()?;

        Ok(())
    }
}
