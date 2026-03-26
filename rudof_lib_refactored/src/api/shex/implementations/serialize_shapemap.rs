use crate::{Result, Rudof, formats::ShapeMapFormat, errors::ShExError};
use shex_ast::{compact::shapemap_compact_printer::ShapemapFormatter, shapemap::QueryShapeMap};
use std::io;

pub fn serialize_shapemap<W: io::Write>(
    rudof: &Rudof,
    shapemap_format: Option<&ShapeMapFormat>,
    show_colors: Option<bool>,
    writer: &mut W,
) -> Result<()> {
    let (shapemap_format, show_colors) = init_defaults(shapemap_format, show_colors);

    let shapemap = rudof.shapemap.as_ref().ok_or(ShExError::NoShapemapLoaded)?;

    match shapemap_format {
        ShapeMapFormat::Compact => {
            serialize_shapemap_compact(shapemap, show_colors, writer)?;
        },
        ShapeMapFormat::Json => {
            serialize_shapemap_json(shapemap, writer)?;
        },
        _ => {
            todo!("Implement serialization for ShapeMap format '{}'", shapemap_format);
        },
    }

    Ok(())
}

fn init_defaults(shapemap_format: Option<&ShapeMapFormat>, show_colors: Option<bool>) -> (ShapeMapFormat, bool) {
    (
        shapemap_format.copied().unwrap_or_default(),
        show_colors.unwrap_or(false),
    )
}

fn serialize_shapemap_compact<W: io::Write>(shapemap: &QueryShapeMap, show_colors: bool, writer: &mut W) -> Result<()> {
    let formatter = match show_colors {
        false => ShapemapFormatter::default().without_colors(),
        true => ShapemapFormatter::default(),
    };

    formatter
        .write_shapemap(shapemap, writer)
        .map_err(|e| ShExError::FailedSerializingShapemap {
            format: "compact".to_string(),
            error: e.to_string(),
        })?;

    Ok(())
}

fn serialize_shapemap_json<W: io::Write>(shapemap: &QueryShapeMap, writer: &mut W) -> Result<()> {
    serde_json::to_writer_pretty(writer, shapemap).map_err(|e| ShExError::FailedSerializingShapemap {
        format: "json".to_string(),
        error: e.to_string(),
    })?;

    Ok(())  
}
