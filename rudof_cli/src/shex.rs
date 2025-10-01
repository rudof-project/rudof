use std::collections::HashMap;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::Instant;

use crate::ShExFormat as CliShExFormat;
use crate::data::get_data_rudof;
use crate::data_format::DataFormat;
use crate::mime_type::MimeType;
use crate::node_selector::{parse_node_selector, parse_shape_selector, start};
use crate::writer::get_writer;
use crate::{ColorSupport, shapemap_format_convert};
use crate::{ResultShExValidationFormat, ShapeMapFormat as CliShapeMapFormat};
use anyhow::Context;
use anyhow::{Result, bail};
use iri_s::IriS;
use rudof_lib::{InputSpec, Rudof, RudofConfig, ShExFormat, ShExFormatter};
use shapemap::ResultShapeMap;
use shex_ast::{Schema, ShapeExprLabel};
use srdf::ReaderMode;
use tracing::trace;

#[allow(clippy::too_many_arguments)]
pub fn run_shex(
    input: &InputSpec,
    schema_format: &CliShExFormat,
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
    let (writer, color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);

    parse_shex_schema_rudof(&mut rudof, input, schema_format, base, reader_mode, config)?;
    if show_schema {
        show_shex_schema_rudof(&rudof, result_schema_format, writer, color)?;
    }
    if show_time {
        let elapsed = begin.elapsed();
        let _ = writeln!(io::stderr(), "elapsed: {:.03?} sec", elapsed.as_secs_f64());
    }
    let schema_resolved = rudof.shex_schema_without_imports()?;
    if config.show_extends() {
        show_extends_table(&mut io::stderr(), schema_resolved.count_extends())?;
    }

    if config.show_imports() {
        writeln!(
            io::stderr(),
            "Local shapes: {}/Total shapes {}",
            schema_resolved.local_shapes_count(),
            schema_resolved.total_shapes_count()
        )?;
    }
    if config.show_shapes() {
        for (shape_label, (_shape_expr, iri)) in schema_resolved.shapes() {
            let label = match shape_label {
                ShapeExprLabel::IriRef { value } => {
                    schema_resolved.resolve_iriref(value).as_str().to_string()
                }
                ShapeExprLabel::BNode { value } => format!("{value}"),
                ShapeExprLabel::Start => "Start".to_string(),
            };
            writeln!(io::stderr(), "{label} from {iri}")?
        }
    }
    if compile && config.show_ir() {
        trace!("Compiling schema to IR...");
        writeln!(io::stdout(), "\nIR:")?;
        if let Some(shex_ir) = rudof.get_shex_ir() {
            trace!("Schema compiled to IR");
            writeln!(io::stdout(), "ShEx IR:")?;
            writeln!(io::stdout(), "{shex_ir}")?;
        } else {
            bail!("Internal error: No ShEx schema read")
        }
    }
    if compile && config.show_dependencies() {
        writeln!(io::stdout(), "\nDependencies:")?;
        if let Some(shex_ir) = rudof.get_shex_ir() {
            for (source, posneg, target) in shex_ir.dependencies() {
                writeln!(io::stdout(), "{source}-{posneg}->{target}")?;
            }
        } else {
            bail!("Internal error: No ShEx schema read")
        }
        writeln!(io::stdout(), "---end dependencies\n")?;
    }
    Ok(())
}

// TODO: Replace by show_schema_rudof
/*pub(crate) fn show_shex_schema(
    schema: &SchemaJson,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    match result_schema_format {
        CliShExFormat::Internal => {
            writeln!(writer, "{schema:?}")?;
            Ok(())
        }
        CliShExFormat::ShExC => {
            let formatter = match color {
                ColorSupport::NoColor => ShExFormatter::default().without_colors(),
                ColorSupport::WithColor => ShExFormatter::default(),
            };
            let str = formatter.format_schema(schema);
            writeln!(writer, "{str}")?;
            Ok(())
        }
        CliShExFormat::ShExJ => {
            let str = serde_json::to_string_pretty(&schema)?;
            writeln!(writer, "{str}")?;
            Ok(())
        }
        CliShExFormat::Simple => {
            let mut simplified = SimpleReprSchema::new();
            simplified.from_schema(schema);
            let str = serde_json::to_string_pretty(&simplified)?;
            writeln!(writer, "{str}")?;
            Ok(())
        }
        _ => bail!("Not implemented conversion to {result_schema_format} yet"),
    }
} */

pub fn show_shex_schema_rudof(
    rudof: &Rudof,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
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
        .open_read(Some(&schema_format.mime_type()), "ShEx schema")
        .context(format!("Get reader from input: {input}"))?;
    let schema_format = shex_format_convert(schema_format);
    let base = get_base(config, base);
    rudof.read_shex(
        reader,
        &schema_format,
        base.as_deref(),
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

fn get_base(config: &RudofConfig, base: &Option<IriS>) -> Option<String> {
    if let Some(base) = base {
        Some(base.to_string())
    } else {
        config
            .shex_config()
            .base
            .as_ref()
            .map(|iri| iri.to_string())
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
        CliShExFormat::Turtle => ShExFormat::Turtle,
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
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    if let Some(schema) = schema {
        let mut rudof = Rudof::new(config);
        let (writer, _color) = get_writer(output, force_overwrite)?;
        let schema_format = schema_format.unwrap_or_default();
        let schema_reader = schema.open_read(Some(&schema_format.mime_type()), "ShEx Schema")?;
        let schema_format = match schema_format {
            CliShExFormat::ShExC => ShExFormat::ShExC,
            CliShExFormat::ShExJ => ShExFormat::ShExJ,
            _ => bail!("ShExJ validation not yet implemented"),
        };
        let base_iri = get_base(config, base_schema);
        let schema_base = base_iri.as_deref();
        rudof.read_shex(
            schema_reader,
            &schema_format,
            schema_base,
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
        write_result_shapemap(writer, &shapemap_format, result)?;
        Ok(())
    } else {
        bail!("No ShEx schema specified")
    }
}

fn write_result_shapemap(
    mut writer: Box<dyn Write + 'static>,
    format: &CliShapeMapFormat,
    result: ResultShapeMap,
) -> Result<()> {
    match format {
        CliShapeMapFormat::Compact => {
            writeln!(writer, "Result:")?;
            result.show_minimal(writer)?;
        }
        CliShapeMapFormat::Internal => {
            let str = serde_json::to_string_pretty(&result)
                .context(format!("Error converting Result to JSON: {result}"))?;
            writeln!(writer, "{str}")?;
        }
    }
    Ok(())
}
