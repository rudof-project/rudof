use std::path::PathBuf;

use crate::ColorSupport;
use crate::writer::get_writer;
use anyhow::Result;
use iri_s::IriS;
use rudof_lib::InputSpec;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::ShapeMapFormatter;
use rudof_lib::shapemap_format::ShapeMapFormat as CliShapeMapFormat;
use shex_ast::shapemap::ShapeMapFormat;

pub fn run_shapemap(
    input: &InputSpec,
    shapemap_format: &CliShapeMapFormat,
    result_format: &CliShapeMapFormat,
    base: &Option<IriS>,
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, color) = get_writer(output, force_overwrite)?;
    let rudof_config = RudofConfig::new()?;
    let mut rudof = Rudof::new(&rudof_config)?;
    let shapemap_format = shapemap_format_convert(shapemap_format);
    let reader = input.open_read(None, "ShapeMap")?;
    rudof.read_shapemap(reader, input.source_name().as_str(), &shapemap_format, base)?;
    let result_format = shapemap_format_convert(result_format);
    let formatter = match color {
        ColorSupport::WithColor => ShapeMapFormatter::default(),
        ColorSupport::NoColor => ShapeMapFormatter::default().without_colors(),
    };
    rudof.serialize_shapemap(Some(&result_format), &formatter, &mut writer)?;
    Ok(())
}

pub fn shapemap_format_convert(shapemap_format: &CliShapeMapFormat) -> ShapeMapFormat {
    match shapemap_format {
        CliShapeMapFormat::Compact => ShapeMapFormat::Compact,
        CliShapeMapFormat::Internal => ShapeMapFormat::Json,
        CliShapeMapFormat::Json => ShapeMapFormat::Json,
        CliShapeMapFormat::Details => ShapeMapFormat::Compact,
        CliShapeMapFormat::Csv => ShapeMapFormat::Csv,
    }
}
