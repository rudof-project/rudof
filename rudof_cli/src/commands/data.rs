use crate::cli::parser::DataArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `data` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Data command logic.
pub struct DataCommand {
    /// Arguments specific to Data command.
    args: DataArgs,
}

impl DataCommand {
    pub fn new(args: DataArgs) -> Self {
        Self { args }
    }
}

impl Command for DataCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "data"
    }

    /// Executes the Data command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let data_format = self.args.data_format.into();
        let reader_mode = self.args.reader_mode.into();
        let result_format = self.args.result_format.into();

        let mut loading = ctx
            .rudof
            .load_data()
            .with_data_format(&data_format)
            .with_reader_mode(&reader_mode);
        if !self.args.data.is_empty() {
            loading = loading.with_data(&self.args.data);
        }
        if let Some(base) = self.args.base.as_deref() {
            loading = loading.with_base(base);
        }
        if let Some(endpoint) = self.args.endpoint.as_deref() {
            loading = loading.with_endpoint(endpoint);
        }
        if !self.args.prefixes.is_empty() {
            loading = loading.with_prefixes(&self.args.prefixes);
        }
        loading.execute()?;

        ctx.rudof
            .serialize_data(&mut ctx.writer)
            .with_result_data_format(&result_format)
            .execute()?;

        Ok(())
    }
}
