use crate::cli::parser::DataArgs;
use crate::commands::base::{Command, CommandContext};
use anyhow::Result;
use rudof_lib::{
    ReaderMode,
    rdf_reader_mode::RDFReaderMode,
    result_data_format::{CheckResultDataFormat, ResultDataFormat, VisualFormat},
    rdf_core::visualizer::{VisualRDFGraph, uml_converter::{ImageFormat, UmlGenerationMode}},
};
use rudof_rdf::rdf_core::visualizer::uml_converter::UmlConverter;

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
        // Convert CLI types to library types
        let reader_mode: RDFReaderMode = (&self.args.reader_mode).into();
        let reader_mode: ReaderMode = reader_mode.into();
        let result_format: ResultDataFormat = (&self.args.result_format).into();
        let check_result_format: CheckResultDataFormat = result_format.try_into().unwrap();

        // Load RDF data into rudof
        ctx.rudof.load_data(
            &self.args.data,
            &(&self.args.data_format).into(),
            &self.args.base,
            &None,
            &reader_mode,
            false,
        )?;

        // Write the RDF data to output
        match check_result_format {
            CheckResultDataFormat::RDFFormat(rdf_format) => {
                ctx.rudof.get_rdf_data().serialize(&rdf_format, &mut ctx.writer)?;
            },
            CheckResultDataFormat::VisualFormat(VisualFormat::PlantUML) => {
                ctx.rudof.data2plant_uml(&mut ctx.writer)?;
            },
            CheckResultDataFormat::VisualFormat(VisualFormat::Svg) | CheckResultDataFormat::VisualFormat(VisualFormat::Png) => {
                let rdf = ctx.rudof.get_rdf_data();
                let uml_converter = VisualRDFGraph::from_rdf(rdf, ctx.rudof.config().rdf_data_config().rdf_visualization_config())?;
                let format = match result_format {
                    ResultDataFormat::Svg => ImageFormat::SVG,
                    ResultDataFormat::Png => ImageFormat::PNG,
                    _ => unreachable!(),
                };
                uml_converter.as_image(&mut ctx.writer, format, &UmlGenerationMode::all(), ctx.rudof.config().plantuml_path())?;
            },
        }

        Ok(())
    }
}
