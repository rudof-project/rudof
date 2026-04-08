use crate::cli::parser::ServiceArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;

/// Implementation of the `service` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Service command logic.
pub struct ServiceCommand {
    /// Arguments specific to Service command.
    args: ServiceArgs,
}

impl ServiceCommand {
    pub fn new(args: ServiceArgs) -> Self {
        Self { args }
    }
}

impl Command for ServiceCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "service"
    }

    /// Executes the Service command logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let reader_mode = self.args.reader_mode.into();
        let format = self.args.service_format.into();
        let result_format = self.args.result_service_format.into();

        let mut load_service_description = ctx
            .rudof
            .load_service_description(&self.args.service)
            .with_data_format(&format)
            .with_reader_mode(&reader_mode);
        if let Some(base) = &self.args.base_data.as_deref() {
            load_service_description = load_service_description.with_base(base);
        }
        load_service_description.execute()?;

        ctx.rudof
            .serialize_service_description(&mut ctx.writer)
            .with_result_service_format(&result_format)
            .execute()?;

        Ok(())
    }
}
