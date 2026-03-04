use crate::cli::parser::ShaclArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::{
    ReaderMode, ShaclFormat as ShaclAstShaclFormat, rdf_reader_mode::RDFReaderMode, shacl_format::ShaclFormat,
};

/// Implementation of the `shacl` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shacl command logic.
pub struct ShaclCommand {
    /// Arguments specific to Shacl command.
    args: ShaclArgs,
}

impl ShaclCommand {
    pub fn new(args: ShaclArgs) -> Self {
        Self { args }
    }
}

impl Command for ShaclCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shacl"
    }

    /// Executes the Shacl command logic.
    #[allow(clippy::unnecessary_fallible_conversions)]
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();
        let schema_format: Option<ShaclFormat> = self.args.shapes_format.as_ref().map(|f| f.into());
        let schema_format: Option<ShaclAstShaclFormat> = schema_format.as_ref().map(|f| f.try_into()).transpose()?;
        let result_shapes_format: ShaclFormat = (&self.args.result_shapes_format).into();
        let result_shapes_format: ShaclAstShaclFormat = result_shapes_format.try_into()?;

        // Load RDF data
        ctx.rudof.load_data(
            &self.args.data,
            &(&self.args.data_format).into(),
            &self.args.base_data,
            &self.args.endpoint,
            &reader_mode,
            true,
        )?;

        // Extract and serialize SHACL schema
        ctx.rudof.shacl_extract(
            &self.args.shapes,
            &schema_format,
            &self.args.base_shapes,
            &reader_mode,
            &result_shapes_format,
            &mut ctx.writer,
        )?;

        Ok(())
    }
}
