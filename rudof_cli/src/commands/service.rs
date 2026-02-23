use crate::cli::parser::ServiceArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use iri_s::MimeType;
use rudof_lib::{data::data_format2rdf_format, rdf_reader_mode::RDFReaderMode, data_format::DataFormat, ReaderMode};

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
        let mut reader = self
            .args
            .service
            .open_read(Some(self.args.service_format.mime_type()), "Service")?;

        let data_format: DataFormat = (&self.args.service_format).into();
        let rdf_format = data_format2rdf_format(&data_format)?;

        let service_config = ctx.rudof.config().service_config();
        let base = service_config.base.as_ref().map(|i| i.as_str());

        let reader_mode:RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode:ReaderMode = reader_mode.into();

        ctx.rudof.read_service_description(
            &mut reader,
            self.args.service.source_name().as_str(),
            &rdf_format,
            base,
            &reader_mode,
        )?;

        Ok(())
    }
}
