use crate::cli::parser::ShaclValidateArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::{Result, anyhow};
use rudof_lib::{
    InMemoryGraph, ReaderMode, ShaclFormat as ShaclAstShaclFormat, ShaclValidationMode, ShapesGraphSource,
    ValidationReport,
    rdf_reader_mode::RDFReaderMode,
    result_shacl_validation_format::{ResultShaclValidationFormat, SortByShaclValidationReport},
    result_shacl_validation_format::{cnv_sort_mode_report, result_format_to_rdf_format},
    shacl_format::ShaclFormat,
    terminal_width::terminal_width,
};
use rudof_rdf::rdf_core::BuildRDF;
use std::io::Write;

/// Implementation of the `shacl-validate` command.
///
/// This struct holds the specific arguments parsed by `clap` and
/// implements the [Command] trait to execute Shacl Validate logic.
pub struct ShaclValidateCommand {
    /// Arguments specific to shacl-validate.
    args: ShaclValidateArgs,
}

impl ShaclValidateCommand {
    pub fn new(args: ShaclValidateArgs) -> Self {
        Self { args }
    }

    fn write_validation_report<W: Write>(
        &self,
        mut writer: W,
        format: &ResultShaclValidationFormat,
        report: ValidationReport,
        sort_by: &SortByShaclValidationReport,
    ) -> Result<(), anyhow::Error> {
        let terminal_width = terminal_width();
        let sort_mode = cnv_sort_mode_report(sort_by);

        match format {
            ResultShaclValidationFormat::Minimal => {
                if report.conforms() {
                    writeln!(writer, "Conforms")?;
                } else {
                    writeln!(
                        writer,
                        "Does not conform, {} violations, {} warnings",
                        report.count_violations(),
                        report.count_warnings()
                    )?;
                }
                Ok(())
            },
            ResultShaclValidationFormat::Compact => {
                report.show_as_table(writer, sort_mode, false, terminal_width)?;
                Ok(())
            },
            ResultShaclValidationFormat::Details => {
                report.show_as_table(writer, sort_mode, true, terminal_width)?;
                Ok(())
            },
            ResultShaclValidationFormat::Json => Err(anyhow!(
                "Generation of JSON for SHACL validation report is not implemented yet"
            )),
            _ => {
                let mut rdf_writer = InMemoryGraph::new();
                report
                    .to_rdf(&mut rdf_writer)
                    .map_err(|e| anyhow!("Error converting SHACL validation report to RDF: {}", e))?;
                let rdf_format = result_format_to_rdf_format(format)?;
                rdf_writer
                    .serialize(&rdf_format, &mut writer)
                    .map_err(|e| anyhow!("Error serializing SHACL validation report to RDF: {}", e))?;
                Ok(())
            },
        }
    }
}

impl Command for ShaclValidateCommand {
    /// Returns the unique identifier for this command.
    fn name(&self) -> &'static str {
        "shacl-validate"
    }

    /// Executes the shacl-validate logic.
    #[allow(clippy::unnecessary_fallible_conversions)]
    fn execute(&self, ctx: &mut CommandContext) -> Result<()> {
        // Convert CLI types to library types
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();
        let schema_format: Option<ShaclFormat> = self.args.shapes_format.as_ref().map(|f| f.into());
        let schema_format: Option<ShaclAstShaclFormat> = schema_format.as_ref().map(|f| f.try_into()).transpose()?;
        let shacl_validation_mode: ShaclValidationMode = (&self.args.mode).into();
        let result_format: ResultShaclValidationFormat = (&self.args.result_format).into();
        let sort_by: SortByShaclValidationReport = (&self.args.sort_by).into();

        // Load RDF data into rudof
        ctx.rudof.load_data(
            &self.args.data,
            &(&self.args.data_format).into(),
            &self.args.base_data,
            &self.args.endpoint,
            &reader_mode,
            false, // allow_no_data = false for validation
        )?;

        // Load SHACL schema if provided, otherwise use data as schema
        let shapes_graph_source = if let Some(ref schema_input) = self.args.shapes {
            // Load schema from external source
            let format = schema_format.unwrap_or_default();
            ctx.rudof
                .load_shacl_schema(schema_input, &format, &self.args.base_shapes, &reader_mode)?;
            ShapesGraphSource::CurrentSchema
        } else {
            // Use current data as schema source
            ShapesGraphSource::CurrentData
        };

        // Perform SHACL validation
        let validation_report = ctx.rudof.validate_shacl(&shacl_validation_mode, &shapes_graph_source)?;

        // Write the validation report to output
        self.write_validation_report(&mut ctx.writer, &result_format, validation_report, &sort_by)?;

        Ok(())
    }
}
