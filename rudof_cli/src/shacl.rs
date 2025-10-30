use std::fmt::Display;
use std::fmt::Formatter;
use std::io::Write;
use std::path::PathBuf;

use anyhow::bail;
use clap::ValueEnum;
use iri_s::IriS;
use rudof_lib::InputSpec;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::ShaclValidationMode;
use rudof_lib::ShapesGraphSource;
use rudof_lib::ValidationReport;
use rudof_lib::data::get_base;
use rudof_lib::data::get_data_rudof;
use rudof_lib::data_format::DataFormat;
use shacl_ast::ShaclFormat;
use shacl_validation::validation_report::report::SortModeReport;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFGraph;
use tracing::Level;
use tracing::debug;
use tracing::enabled;
use tracing::trace;

use crate::CliShaclFormat;
use crate::ResultShaclValidationFormat;
use crate::terminal_width;
use crate::writer::get_writer;
use anyhow::Result;
use iri_s::mime_type::MimeType;

#[allow(clippy::too_many_arguments)]
pub fn run_shacl(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base_data: &Option<IriS>,
    endpoint: &Option<String>,
    schema: &Option<InputSpec>,
    shapes_format: &Option<CliShaclFormat>,
    base_shapes: &Option<IriS>,
    result_shapes_format: &CliShaclFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        base_data,
        endpoint,
        reader_mode,
        config,
        true,
    )?;
    if let Some(schema) = schema {
        let shapes_format = (*shapes_format).unwrap_or_default();
        add_shacl_schema_rudof(
            &mut rudof,
            schema,
            &shapes_format,
            base_shapes,
            reader_mode,
            config,
        )?;
        rudof.compile_shacl(&ShapesGraphSource::current_schema())
    } else {
        rudof.compile_shacl(&ShapesGraphSource::current_data())
    }?;

    let shacl_format = shacl_format_convert(*result_shapes_format)?;
    rudof.serialize_shacl(&shacl_format, &mut writer)?;
    if enabled!(Level::DEBUG) {
        match rudof.get_shacl_ir() {
            Some(ir) => debug!("SHACL IR: {}", ir),
            None => debug!("No SHACL IR available"),
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn run_shacl_convert(
    input: &InputSpec,
    input_format: &CliShaclFormat,
    base: &Option<IriS>,
    output: &Option<PathBuf>,
    output_format: &CliShaclFormat,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    let mime_type = input_format.mime_type();
    let reader = input.open_read(Some(mime_type), "SHACL shapes")?;
    let input_format = shacl_format_convert(*input_format)?;
    let base = get_base(input, config, base)?;
    rudof.read_shacl(
        reader,
        &input.to_string(),
        &input_format,
        base.as_deref(),
        reader_mode,
    )?;
    let output_format = shacl_format_convert(*output_format)?;
    rudof.serialize_shacl(&output_format, &mut writer)?;
    Ok(())
}

pub fn add_shacl_schema_rudof(
    rudof: &mut Rudof,
    schema: &InputSpec,
    shapes_format: &CliShaclFormat,
    base_shapes: &Option<IriS>,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let mime_type = shapes_format.mime_type();
    let reader = schema.open_read(Some(mime_type), "SHACL shapes")?;
    let reader_name = schema.to_string();
    let shapes_format = shacl_format_convert(*shapes_format)?;
    let base = get_base(schema, config, base_shapes)?;
    rudof.read_shacl(
        reader,
        &reader_name,
        &shapes_format,
        base.as_deref(),
        reader_mode,
    )?;
    Ok(())
}

fn shacl_format_convert(shacl_format: CliShaclFormat) -> Result<ShaclFormat> {
    match shacl_format {
        CliShaclFormat::Turtle => Ok(ShaclFormat::Turtle),
        CliShaclFormat::RDFXML => Ok(ShaclFormat::RDFXML),
        CliShaclFormat::NTriples => Ok(ShaclFormat::NTriples),
        CliShaclFormat::TriG => Ok(ShaclFormat::TriG),
        CliShaclFormat::N3 => Ok(ShaclFormat::N3),
        CliShaclFormat::NQuads => Ok(ShaclFormat::NQuads),
        CliShaclFormat::Internal => Ok(ShaclFormat::Internal),
        CliShaclFormat::JsonLd => Ok(ShaclFormat::JsonLd),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_validate_shacl(
    schema: &Option<InputSpec>,
    shapes_format: &Option<CliShaclFormat>,
    base_shapes: &Option<IriS>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base_data: &Option<IriS>,
    endpoint: &Option<String>,
    reader_mode: &ReaderMode,
    mode: ShaclValidationMode,
    _debug: u8,
    result_format: &ResultShaclValidationFormat,
    sort_by: &SortByShaclValidationReport,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config)?;
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        base_data,
        endpoint,
        reader_mode,
        config,
        false,
    )?;
    let validation_report = if let Some(schema) = schema {
        let shapes_format = (*shapes_format).unwrap_or_default();
        add_shacl_schema_rudof(
            &mut rudof,
            schema,
            &shapes_format,
            base_shapes,
            reader_mode,
            config,
        )?;
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_schema())
    } else {
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_data())
    }?;
    write_validation_report(writer, result_format, validation_report, sort_by)?;
    Ok(())
}

fn write_validation_report(
    mut writer: Box<dyn Write + 'static>,
    format: &ResultShaclValidationFormat,
    report: ValidationReport,
    sort_by: &SortByShaclValidationReport,
) -> Result<()> {
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
        }
        ResultShaclValidationFormat::Compact => {
            report.show_as_table(writer, sort_mode, false, terminal_width)?;
            Ok(())
        }
        ResultShaclValidationFormat::Details => {
            report.show_as_table(writer, sort_mode, true, terminal_width)?;
            Ok(())
        }
        ResultShaclValidationFormat::Json => {
            bail!("Generation of JSON for SHACl validation report is not implemented yet")
            /*let str = serde_json::to_string_pretty(&report)
                .context("Error converting Result to JSON: {result}")?;
            writeln!(writer, "{str}")?;*/
        }
        _ => {
            use srdf::BuildRDF;
            let mut rdf_writer = SRDFGraph::new();
            report.to_rdf(&mut rdf_writer)?;
            let rdf_format = result_format_to_rdf_format(format)?;
            rdf_writer.serialize(&rdf_format, &mut writer)?;
            Ok(())
        }
    }
}

fn cnv_sort_mode_report(sort_by: &SortByShaclValidationReport) -> SortModeReport {
    match sort_by {
        SortByShaclValidationReport::Severity => SortModeReport::Severity,
        SortByShaclValidationReport::Node => SortModeReport::Node,
        SortByShaclValidationReport::Component => SortModeReport::Component,
        SortByShaclValidationReport::Value => SortModeReport::Value,
        SortByShaclValidationReport::Path => SortModeReport::Path,
        SortByShaclValidationReport::SourceShape => SortModeReport::Source,
        SortByShaclValidationReport::Details => SortModeReport::Details,
    }
}

fn result_format_to_rdf_format(result_format: &ResultShaclValidationFormat) -> Result<RDFFormat> {
    match result_format {
        ResultShaclValidationFormat::Turtle => Ok(RDFFormat::Turtle),
        ResultShaclValidationFormat::NTriples => Ok(RDFFormat::NTriples),
        ResultShaclValidationFormat::RDFXML => Ok(RDFFormat::RDFXML),
        ResultShaclValidationFormat::TriG => Ok(RDFFormat::TriG),
        ResultShaclValidationFormat::N3 => Ok(RDFFormat::N3),
        ResultShaclValidationFormat::NQuads => Ok(RDFFormat::NQuads),
        _ => bail!("Unsupported result format {result_format}"),
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum SortByShaclValidationReport {
    #[default]
    Severity,
    Node,
    Component,
    Value,
    Path,
    SourceShape,
    Details,
}

impl Display for SortByShaclValidationReport {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SortByShaclValidationReport::Severity => write!(dest, "severity"),
            SortByShaclValidationReport::Node => write!(dest, "node"),
            SortByShaclValidationReport::Component => write!(dest, "component"),
            SortByShaclValidationReport::Value => write!(dest, "value"),
            SortByShaclValidationReport::Path => write!(dest, "path"),
            SortByShaclValidationReport::SourceShape => write!(dest, "sourceShape"),
            SortByShaclValidationReport::Details => write!(dest, "details"),
        }
    }
}
