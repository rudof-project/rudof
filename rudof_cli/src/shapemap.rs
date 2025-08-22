use std::path::PathBuf;

use crate::ColorSupport;
use crate::InputSpec;
use crate::ShapeMapFormat as CliShapeMapFormat;
use crate::writer::get_writer;
use anyhow::Result;
use rudof_lib::Rudof;
use rudof_lib::RudofConfig;
use rudof_lib::ShapeMapFormatter;
use shapemap::ShapeMapFormat;

pub fn run_shapemap(
    shapemap: &InputSpec,
    shapemap_format: &CliShapeMapFormat,
    result_format: &CliShapeMapFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(&RudofConfig::new());
    let shapemap_format = shapemap_format_convert(shapemap_format);
    rudof.read_shapemap(shapemap.open_read(None, "ShapeMap")?, &shapemap_format)?;
    let result_format = shapemap_format_convert(result_format);
    let formatter = match color {
        ColorSupport::WithColor => ShapeMapFormatter::default(),
        ColorSupport::NoColor => ShapeMapFormatter::default().without_colors(),
    };
    rudof.serialize_shapemap(&result_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn shapemap_format_convert(shapemap_format: &CliShapeMapFormat) -> ShapeMapFormat {
    match shapemap_format {
        CliShapeMapFormat::Compact => ShapeMapFormat::Compact,
        CliShapeMapFormat::Internal => ShapeMapFormat::JSON,
    }
}
