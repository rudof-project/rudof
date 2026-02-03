use crate::{
    result_shex_validation_format::ResultShExValidationFormat, selector::*, shapemap_format::ShapeMapFormat as CliShapeMapFormat, shex_format::ShExFormat as CliShExFormat,
    sort_by_result_shape_map::SortByResultShapeMap, terminal_width::terminal_width,
    InputSpec, Rudof,
    RudofConfig, RudofError,
};
use iri_s::IriS;
use iri_s::MimeType;
use shex_ast::shapemap::ResultShapeMap;
use shex_ast::ShExFormat;
use srdf::RDFFormat;
use srdf::ReaderMode;
use std::env;
use std::io::Write;
use tracing::info;
use url::Url;

#[allow(clippy::too_many_arguments)]
pub fn validate_shex<W: Write>(
    rudof: &mut Rudof,
    schema: &Option<InputSpec>,
    schema_format: &Option<CliShExFormat>,
    base_schema: &Option<IriS>,
    reader_mode: &ReaderMode,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap: &Option<InputSpec>,
    shapemap_format: &CliShapeMapFormat,
    result_format: &ResultShExValidationFormat,
    sort_by: &SortByResultShapeMap,
    config: &RudofConfig,
    writer: &mut W,
) -> Result<(), RudofError> {
    if let Some(schema) = schema {
        let schema_format = schema_format.unwrap_or_default();
        let schema_reader = schema
            .open_read(Some(schema_format.mime_type()), "ShEx Schema")
            .map_err(|e| RudofError::ReadingPathContext {
                path: schema.source_name().to_string(),
                error: e.to_string(),
                context: "ShEx Schema".to_string(),
            })?;
        let schema_format = schema_format.try_into()?;

        let base_iri = get_base(config, base_schema)?;

        rudof.read_shex(
            schema_reader,
            &schema_format,
            Some(base_iri.as_str()),
            reader_mode,
            Some(&schema.source_name()),
        )?;

        let shapemap_format = shapemap_format.into();

        if let Some(shapemap_spec) = shapemap {
            let shapemap_reader = shapemap_spec.open_read(None, "ShapeMap").map_err(|e| {
                RudofError::ShapeMapParseError {
                    source_name: shapemap_spec.source_name(),
                    str: shapemap_spec.source_name().to_string(),
                    error: e.to_string(),
                }
            })?;

            rudof.read_shapemap(
                shapemap_reader,
                shapemap_spec.source_name().as_str(),
                &shapemap_format,
            )?;
        }

        // If individual node/shapes are declared add them to current shape map
        match (maybe_node, maybe_shape) {
            (None, None) => {
                // Nothing to do in this case
            }
            (Some(node_str), None) => {
                let node_selector = parse_node_selector(node_str)?;
                rudof.shapemap_add_node_shape_selectors(node_selector, start())
            }
            (Some(node_str), Some(shape_str)) => {
                let node_selector = parse_node_selector(node_str)?;
                let shape_selector = parse_shape_selector(shape_str)?;
                rudof.shapemap_add_node_shape_selectors(node_selector, shape_selector);
            }
            (None, Some(shape_str)) => {
                tracing::debug!(
                    "Shape label {shape_str} ignored because noshapemap has also been provided"
                )
            }
        };

        let result = rudof.validate_shex()?;

        let shapemap_format = result_format.try_into()?;
        write_result_shapemap(writer, &shapemap_format, result, sort_by)?;
        Ok(())
    } else {
        Err(RudofError::NoShExSchemaToSerialize)
    }
}

fn get_base(config: &RudofConfig, base: &Option<IriS>) -> Result<IriS, RudofError> {
    if let Some(base) = base {
        Ok(base.clone())
    } else if let Some(base) = config.shex_config().base.as_ref() {
        Ok(base.clone())
    } else {
        let cwd = env::current_dir().map_err(|e| RudofError::CurrentDirError {
            error: format!("{e}"),
        })?;
        // Note: we use from_directory_path to convert a directory to a file URL that ends with a trailing slash
        // from_url_path would not add the trailing slash and would fail when resolving relative IRIs
        let url =
            Url::from_directory_path(&cwd).map_err(|_| RudofError::ConvertingCurrentFolderUrl {
                current_dir: cwd.to_string_lossy().to_string(),
            })?;
        Ok(url.into())
    }
}

fn write_result_shapemap<W: Write>(
    mut writer: W,
    format: &CliShapeMapFormat,
    result: ResultShapeMap,
    sort_by: &SortByResultShapeMap,
) -> Result<(), RudofError> {
    match format {
        CliShapeMapFormat::Compact => {
            writeln!(writer, "Result:")?;
            result.as_table(writer, sort_by.into(), false, terminal_width())?;
        }
        CliShapeMapFormat::CSV => {
            info!("Serializing result as CSV");
            result.as_csv(writer, sort_by.into(), true)?;
        }
        CliShapeMapFormat::Internal => {
            let str = serde_json::to_string_pretty(&result).map_err(|e| RudofError::Generic {
                error: e.to_string(),
            })?;
            writeln!(writer, "{str}")?;
        }
        CliShapeMapFormat::Json => {
            let str = serde_json::to_string_pretty(&result).map_err(|e| RudofError::Generic {
                error: e.to_string(),
            })?;
            writeln!(writer, "{str}")?;
        }
        CliShapeMapFormat::Details => {
            writeln!(writer, "Result:")?;
            result.as_table(writer, sort_by.into(), true, terminal_width())?;
        }
    }
    Ok(())
}

/// Parse a ShEx schema from the given `InputSpec` into the provided `Rudof` instance.
/// This centralizes the parsing and base resolution logic so callers (CLI, MCP)
/// don't duplicate behavior.
pub fn parse_shex_schema(
    rudof: &mut Rudof,
    input: &InputSpec,
    schema_format: &CliShExFormat,
    base: &Option<IriS>,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<(), RudofError> {
    let reader = input
        .open_read(Some(schema_format.mime_type()), "ShEx schema")
        .map_err(|e| RudofError::ReadingPathContext {
            path: input.source_name().to_string(),
            error: e.to_string(),
            context: "ShEx schema".to_string(),
        })?;

    let shex_format = shex_format_convert(schema_format);
    let base_iri = get_base(config, base)?;

    rudof.read_shex(
        reader,
        &shex_format,
        Some(base_iri.as_str()),
        reader_mode,
        Some(&input.source_name()),
    )?;

    if config.shex_config().check_well_formed() {
        let shex_ir = rudof.get_shex_ir().unwrap();
        if shex_ir.has_neg_cycle() {
            return Err(RudofError::Generic {
                error: format!(
                    "Schema contains negative cycles: {:?}",
                    shex_ir.neg_cycles()
                ),
            });
        }
    }

    Ok(())
}

pub fn shex_format_convert(shex_format: &CliShExFormat) -> ShExFormat {
    match shex_format {
        CliShExFormat::ShExC => ShExFormat::ShExC,
        CliShExFormat::ShExJ => ShExFormat::ShExJ,
        CliShExFormat::Turtle => ShExFormat::RDFFormat(RDFFormat::Turtle),
        _ => ShExFormat::ShExC,
    }
}

/// Serialize the current ShEx (full schema) from `rudof` into `writer`.
pub fn serialize_current_shex_rudof<W: Write>(
    rudof: &Rudof,
    result_schema_format: &CliShExFormat,
    formatter: &shex_ast::compact::ShExFormatter,
    writer: &mut W,
) -> Result<(), RudofError> {
    let shex_format = shex_format_convert(result_schema_format);
    rudof.serialize_current_shex(&shex_format, formatter, writer)?;
    Ok(())
}

/// Serialize a selected shape from the current ShEx in `rudof` into `writer`.
pub fn serialize_shape_current_shex_rudof<W: Write>(
    rudof: &Rudof,
    shape: &shex_ast::shapemap::ShapeSelector,
    result_schema_format: &CliShExFormat,
    formatter: &shex_ast::compact::ShExFormatter,
    writer: &mut W,
) -> Result<(), RudofError> {
    let shex_format = shex_format_convert(result_schema_format);
    rudof.serialize_shape_current_shex(shape, &shex_format, formatter, writer)?;
    Ok(())
}

/// Serialize a provided `Schema` value using `rudof` helpers into `writer`.
pub fn serialize_shex_rudof<W: Write>(
    rudof: &Rudof,
    shex: &shex_ast::Schema,
    result_schema_format: &CliShExFormat,
    formatter: &shex_ast::compact::ShExFormatter,
    writer: &mut W,
) -> Result<(), RudofError> {
    let shex_format = (*result_schema_format).try_into()?;
    rudof.serialize_shex(shex, &shex_format, formatter, writer)?;
    Ok(())
}
