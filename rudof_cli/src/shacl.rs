use std::path::PathBuf;

use iri_s::IriS;
use rudof_lib::InputSpec;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::ShaclValidationMode;
use rudof_lib::ShapesGraphSource;
use rudof_lib::data::get_base;
use rudof_lib::data::get_data_rudof;
use rudof_lib::data_format::DataFormat;
use srdf::ReaderMode;
use tracing::Level;
use tracing::debug;
use tracing::enabled;
use tracing::trace;

use rudof_lib::{shacl_format::CliShaclFormat, shacl::*, result_shacl_validation_format::{ResultShaclValidationFormat, SortByShaclValidationReport}};
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
        trace!("Compiling SHACL schema from shapes graph");
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
    let mut reader = input.open_read(Some(mime_type), "SHACL shapes")?;
    let input_format = shacl_format_convert(*input_format)?;
    let base = get_base(input, config, base)?;
    rudof.read_shacl(
        &mut reader,
        &input.to_string(),
        &input_format,
        base.as_deref(),
        reader_mode,
    )?;
    let output_format = shacl_format_convert(*output_format)?;
    rudof.serialize_shacl(&output_format, &mut writer)?;
    Ok(())
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
