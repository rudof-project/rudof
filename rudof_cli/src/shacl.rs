use std::io::Write;
use std::path::PathBuf;

use anyhow::bail;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::ShaclValidationMode;
use rudof_lib::ShapesGraphSource;
use rudof_lib::ValidationReport;
use shacl_ast::ShaclFormat;
use srdf::RDFFormat;
use srdf::ReaderMode;
use srdf::SRDFGraph;
use tracing::debug;
use tracing::enabled;
use tracing::Level;

use crate::data::get_base;
use crate::data::get_data_rudof;
use crate::data_format::DataFormat;
use crate::mime_type::MimeType;
use crate::writer::get_writer;
use crate::CliShaclFormat;
use crate::InputSpec;
use crate::RDFReaderMode;
use crate::ResultShaclValidationFormat;
use anyhow::Result;

pub fn run_shacl(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    schema: &Option<InputSpec>,
    shapes_format: &Option<CliShaclFormat>,
    result_shapes_format: &CliShaclFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        endpoint,
        reader_mode,
        config,
        true,
    )?;
    if let Some(schema) = schema {
        let reader_mode = (*reader_mode).into();
        let shapes_format = (*shapes_format).unwrap_or_default();
        add_shacl_schema_rudof(&mut rudof, schema, &shapes_format, &reader_mode, config)?;
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

pub fn run_shacl_convert(
    input: &InputSpec,
    input_format: &CliShaclFormat,
    output: &Option<PathBuf>,
    output_format: &CliShaclFormat,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    let mime_type = input_format.mime_type();
    let mime_type_str = mime_type.as_str();
    let reader = input.open_read(Some(mime_type_str), "SHACL shapes")?;
    let input_format = shacl_format_convert(*input_format)?;
    let base = get_base(input, config)?;
    rudof.read_shacl(
        reader,
        &input_format,
        base.as_deref(),
        &(*reader_mode).into(),
    )?;
    let output_format = shacl_format_convert(*output_format)?;
    rudof.serialize_shacl(&output_format, &mut writer)?;
    Ok(())
}

pub fn add_shacl_schema_rudof(
    rudof: &mut Rudof,
    schema: &InputSpec,
    shapes_format: &CliShaclFormat,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let mime_type = shapes_format.mime_type();
    let mime_type_str = mime_type.as_str();
    let reader = schema.open_read(Some(mime_type_str), "SHACL shapes")?;
    let shapes_format = shacl_format_convert(*shapes_format)?;
    let base = get_base(schema, config)?;
    rudof.read_shacl(reader, &shapes_format, base.as_deref(), reader_mode)?;
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
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_validate_shacl(
    schema: &Option<InputSpec>,
    shapes_format: &Option<CliShaclFormat>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    mode: ShaclValidationMode,
    _debug: u8,
    result_format: &ResultShaclValidationFormat,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(
        &mut rudof,
        data,
        data_format,
        endpoint,
        reader_mode,
        config,
        false,
    )?;
    let validation_report = if let Some(schema) = schema {
        let reader_mode = (*reader_mode).into();
        let shapes_format = (*shapes_format).unwrap_or_default();
        add_shacl_schema_rudof(&mut rudof, schema, &shapes_format, &reader_mode, config)?;
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_schema())
    } else {
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_data())
    }?;
    write_validation_report(writer, result_format, validation_report)?;
    Ok(())
}

fn write_validation_report(
    mut writer: Box<dyn Write + 'static>,
    format: &ResultShaclValidationFormat,
    report: ValidationReport,
) -> Result<()> {
    match format {
        ResultShaclValidationFormat::Compact => {
            writeln!(writer, "Validation report: {report}")?;
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
        }
    }
    Ok(())
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
