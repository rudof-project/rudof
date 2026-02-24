use crate::cli::parser::ShexValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Result, anyhow};
use rudof_lib::{
    ReaderMode, ShExFormat as ShExAstShExFormat, rdf_reader_mode::RDFReaderMode,
    result_shex_validation_format::ResultShExValidationFormat, shapemap_format::ShapeMapFormat,
    shex_format::ShExFormat, sort_by_result_shape_map::SortByResultShapeMap, terminal_width::terminal_width,
};
use std::io::Write;

/// Implementation of the `shex-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute ShexValidate logic.
pub struct ShexValidateCommand {
    /// Arguments specific to shex-validate.
    args: ShexValidateArgs,
}

impl ShexValidateCommand {
    pub fn new(args: ShexValidateArgs) -> Self {
        Self { args }
    }

    /// Writes the validation result shapemap in the requested format.
    ///
    /// This function remains in the CLI layer because it deals with output formatting,
    /// which is a CLI concern.
    fn write_result_shapemap<W: Write>(
        &self,
        mut writer: W,
        format: &ShapeMapFormat,
        result: rudof_lib::ResultShapeMap,
        sort_by: &SortByResultShapeMap,
    ) -> Result<()> {
        match format {
            ShapeMapFormat::Compact => {
                writeln!(writer, "Result:")?;
                result.as_table(writer, sort_by.into(), false, terminal_width())?;
            },
            ShapeMapFormat::Csv => {
                result.as_csv(writer, sort_by.into(), true)?;
            },
            ShapeMapFormat::Internal | ShapeMapFormat::Json => {
                let str = serde_json::to_string_pretty(&result)
                    .map_err(|e| anyhow!("JSON serialization error: {}", e))?;
                writeln!(writer, "{str}")?;
            },
            ShapeMapFormat::Details => {
                writeln!(writer, "Result:")?;
                result.as_table(writer, sort_by.into(), true, terminal_width())?;
            },
        }
        Ok(())
    }
}

impl Command for ShexValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shex-validate"
    }

    /// Executes the shex-validate logic.
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // Convert CLI types to library types
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();
        let schema_format: Option<ShExFormat> = self.args.schema_format.as_ref().map(|f| f.into());
        let schema_format: Option<ShExAstShExFormat> = schema_format.as_ref().map(|f| f.try_into()).transpose()?;
        let shapemap_format: ShapeMapFormat = (&self.args.shapemap_format).into();
        let result_format: ResultShExValidationFormat = (&self.args.result_format).into();
        let sort_by: SortByResultShapeMap = (&self.args.sort_by).into();

        // Load RDF data into rudof
        ctx.rudof.load_data(
            &self.args.data,
            &(&self.args.data_format).into(),
            &self.args.base_data,
            &self.args.endpoint,
            &reader_mode,
            false, // allow_no_data = false for validation
        )?;

        // Perform complete ShEx validation using the new high-level method
        let result = ctx
            .rudof
            .validate_shex_complete(
                &self.args.schema,
                &schema_format,
                &self.args.base_schema,
                &reader_mode,
                &self.args.shapemap,
                &shapemap_format.into(),
                &self.args.node,
                &self.args.shape,
            )
            .map_err(anyhow::Error::from)?;

        // Write results in the requested format
        self.write_result_shapemap(&mut ctx.writer, &(&result_format).try_into()?, result, &sort_by)?;

        Ok(())
    }
}
