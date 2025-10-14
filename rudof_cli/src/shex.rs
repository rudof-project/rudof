use crate::data::get_data_rudof;
use crate::data_format::DataFormat;
use crate::node_selector::{parse_node_selector, parse_shape_selector, start};
use crate::writer::get_writer;
use crate::{ColorSupport, shapemap_format_convert, terminal_width};
use crate::{ResultShExValidationFormat, ShapeMapFormat as CliShapeMapFormat};
use crate::{ShExFormat as CliShExFormat, SortByResultShapeMap};
use anyhow::Context;
use anyhow::{Result, bail};
use iri_s::IriS;
use iri_s::mime_type::MimeType;
use rudof_lib::{InputSpec, Rudof, RudofConfig, RudofError, ShExFormatter};
use shex_ast::shapemap::{ResultShapeMap, ShapeSelector};
use shex_ast::{Schema, ShExFormat};
use srdf::{RDFFormat, ReaderMode};
use std::collections::HashMap;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;
use tracing::trace;
use url::Url;

#[allow(clippy::too_many_arguments)]
pub fn run_shex(
    input: &InputSpec,
    schema_format: &CliShExFormat,
    shape: &Option<String>,
    base: &Option<IriS>,
    result_schema_format: &CliShExFormat,
    output: &Option<PathBuf>,
    show_time: bool,
    show_schema: bool,
    compile: bool,
    force_overwrite: bool,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let begin = Instant::now();
    let (mut writer, color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);

    parse_shex_schema_rudof(&mut rudof, input, schema_format, base, reader_mode, config)?;
    if let Some(shape_label) = shape {
        let shape_selector = parse_shape_selector(shape_label)?;
        show_shape_shex_schema_rudof(
            &rudof,
            &shape_selector,
            result_schema_format,
            &mut *writer,
            color.clone(),
        )?;
    }
    if show_schema {
        show_shex_schema_rudof(&rudof, result_schema_format, &mut *writer, color)?;
    }
    if show_time {
        let elapsed = begin.elapsed();
        let _ = writeln!(io::stdout(), "elapsed: {:.03?} sec", elapsed.as_secs_f64());
    }
    if compile && config.show_ir() {
        if let Some(shex_ir) = rudof.get_shex_ir() {
            trace!("Schema compiled to IR");
            writeln!(io::stdout(), "ShEx Internal Representation:")?;
            writeln!(io::stdout(), "{shex_ir}")?;

            if config.show_extends() {
                show_extends_table(&mut io::stderr(), shex_ir.count_extends())?;
            }

            if config.show_imports() {
                writeln!(
                    &mut writer,
                    "Local shapes: {}/Total shapes {}",
                    shex_ir.local_shapes_count(),
                    shex_ir.total_shapes_count()
                )?;
            }
            if config.show_shapes() {
                for (label, source, _expr) in shex_ir.shapes() {
                    writeln!(
                        &mut writer,
                        "{label}{}",
                        if shex_ir.imported_schemas().is_empty() {
                            String::new()
                        } else {
                            format!(" from {source}")
                        }
                    )?;
                }
            }
            if config.show_dependencies() {
                writeln!(&mut writer, "\nDependencies:")?;
                for (source, posneg, target) in shex_ir.dependencies() {
                    writeln!(io::stdout(), "{source}-{posneg}->{target}")?;
                }
                writeln!(&mut writer, "---end dependencies\n")?;
            }
        } else {
            bail!("Internal error: Schema was not compiled to IR")
        }
        Ok(())
    } else {
        Ok(())
    }
}

pub fn show_shex_schema_rudof(
    rudof: &Rudof,
    result_schema_format: &CliShExFormat,
    mut writer: &mut dyn Write,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_current_shex(&shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn show_shape_shex_schema_rudof(
    rudof: &Rudof,
    shape: &ShapeSelector,
    result_schema_format: &CliShExFormat,
    mut writer: &mut dyn Write,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_shape_current_shex(shape, &shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn show_shex_schema(
    rudof: &Rudof,
    shex: &Schema,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    let shex_format = shex_format_convert(result_schema_format);
    let formatter = match color {
        ColorSupport::NoColor => ShExFormatter::default().without_colors(),
        ColorSupport::WithColor => ShExFormatter::default(),
    };
    rudof.serialize_shex(shex, &shex_format, &formatter, &mut writer)?;
    Ok(())
}

pub fn parse_shex_schema_rudof(
    rudof: &mut Rudof,
    input: &InputSpec,
    schema_format: &CliShExFormat,
    base: &Option<IriS>,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let reader = input
        .open_read(Some(schema_format.mime_type()), "ShEx schema")
        .context(format!("Get reader from input: {input}"))?;
    let schema_format = shex_format_convert(schema_format);
    let base = get_base(config, base)?;
    rudof.read_shex(
        reader,
        &schema_format,
        Some(base.as_str()),
        reader_mode,
        Some(&input.source_name()),
    )?;
    if config.shex_config().check_well_formed() {
        let shex_ir = rudof.get_shex_ir().unwrap();
        if shex_ir.has_neg_cycle() {
            let neg_cycles = shex_ir.neg_cycles();
            bail!("Schema contains negative cycles: {neg_cycles:?}");
        }
    }
    Ok(())
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
        let iri = IriS::from_url(&url);
        Ok(iri)
    }
}

fn show_extends_table<R: Write>(
    writer: &mut R,
    extends_count: HashMap<usize, usize>,
) -> Result<()> {
    for (key, value) in extends_count.iter() {
        writeln!(writer, "Shapes with {key} extends = {value}")?;
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

#[allow(clippy::too_many_arguments)]
pub fn run_validate_shex(
    schema: &Option<InputSpec>,
    schema_format: &Option<CliShExFormat>,
    base_schema: &Option<IriS>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    base_data: &Option<IriS>,
    endpoint: &Option<String>,
    reader_mode: &ReaderMode,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap: &Option<InputSpec>,
    shapemap_format: &CliShapeMapFormat,
    _debug: u8,
    result_format: &ResultShExValidationFormat,
    sort_by: &SortByResultShapeMap,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    if let Some(schema) = schema {
        let mut rudof = Rudof::new(config);
        let (writer, _color) = get_writer(output, force_overwrite)?;
        let schema_format = schema_format.unwrap_or_default();
        let schema_reader = schema.open_read(Some(schema_format.mime_type()), "ShEx Schema")?;
        let schema_format = match schema_format {
            CliShExFormat::ShExC => ShExFormat::ShExC,
            CliShExFormat::ShExJ => ShExFormat::ShExJ,
            CliShExFormat::JSON => ShExFormat::ShExJ,
            CliShExFormat::JSONLD => ShExFormat::ShExJ,
            _ => bail!("ShExJ validation not yet implemented"),
        };
        let base_iri = get_base(config, base_schema)?;
        rudof.read_shex(
            schema_reader,
            &schema_format,
            Some(base_iri.as_str()),
            reader_mode,
            Some(&schema.source_name()),
        )?;
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

        let shapemap_format = shapemap_format_convert(shapemap_format);
        if let Some(shapemap_spec) = shapemap {
            let shapemap_reader = shapemap_spec.open_read(None, "ShapeMap")?;
            rudof.read_shapemap(shapemap_reader, &shapemap_format)?;
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
        let shapemap_format = result_format.to_shapemap_format()?;
        write_result_shapemap(writer, &shapemap_format, result, sort_by)?;
        Ok(())
    } else {
        bail!("No ShEx schema specified")
    }
}

fn write_result_shapemap(
    mut writer: Box<dyn Write + 'static>,
    format: &CliShapeMapFormat,
    result: ResultShapeMap,
    sort_by: &SortByResultShapeMap,
) -> Result<()> {
    match format {
        CliShapeMapFormat::Compact => {
            writeln!(writer, "Result:")?;
            result.show_as_table(writer, cnv_sort_mode(sort_by), false, terminal_width())?;
        }
        CliShapeMapFormat::Internal => {
            let str = serde_json::to_string_pretty(&result)
                .context("Error converting Result to JSON".to_string())?;
            writeln!(writer, "{str}")?;
        }
        CliShapeMapFormat::Json => {
            let str = serde_json::to_string_pretty(&result)
                .context("Error converting Result to JSON".to_string())?;
            writeln!(writer, "{str}")?;
        }
        CliShapeMapFormat::Details => {
            writeln!(writer, "Result:")?;
            result.show_as_table(writer, cnv_sort_mode(sort_by), true, terminal_width())?;
        }
    }
    Ok(())
}

fn cnv_sort_mode(sort_by: &SortByResultShapeMap) -> shex_ast::shapemap::result_shape_map::SortMode {
    match sort_by {
        SortByResultShapeMap::Node => shex_ast::shapemap::result_shape_map::SortMode::Node,
        SortByResultShapeMap::Shape => shex_ast::shapemap::result_shape_map::SortMode::Shape,
        SortByResultShapeMap::Status => shex_ast::shapemap::result_shape_map::SortMode::Status,
        SortByResultShapeMap::Details => shex_ast::shapemap::result_shape_map::SortMode::Details,
    }
}
