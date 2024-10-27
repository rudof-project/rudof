extern crate anyhow;
extern crate clap;
extern crate dctap;
extern crate iri_s;
extern crate oxrdf;
extern crate prefixmap;
extern crate regex;
extern crate serde_json;
extern crate shacl_ast;
extern crate shapemap;
extern crate shapes_converter;
extern crate srdf;
extern crate supports_color;
extern crate tracing;
extern crate tracing_subscriber;

use anyhow::*;
use clap::Parser;
use cli::{
    Cli, Command, DCTapFormat, DCTapResultFormat, DataFormat, InputConvertMode, MimeType,
    OutputConvertMode, RDFReaderMode, ResultQueryFormat, ResultServiceFormat, ShowNodeMode,
    ValidationMode,
};
use dctap::{DCTap, DCTapConfig, TapConfig};
use iri_s::IriS;
use prefixmap::{IriRef, PrefixMap};
use rudof_lib::{
    Rudof, RudofConfig, ShExFormat, ShExFormatter, ShaclFormat, ShaclValidationMode,
    ShapeMapParser, ShapemapFormatter, ShapesGraphSource,
};
use shacl_ast::ShaclWriter;
use shapemap::{NodeSelector, ShapeMapFormat as ShapemapFormat, ShapeSelector};
use shapes_converter::ShEx2Sparql;
use shapes_converter::{ImageFormat, ShEx2Html, ShEx2Uml, Shacl2ShEx, Tap2ShEx, UmlGenerationMode};
use shex_ast::object_value::ObjectValue;
use shex_ast::{ShapeExprLabel, SimpleReprSchema};
use sparql_service::{QueryConfig, RdfData, ServiceDescription};
use srdf::srdf_graph::SRDFGraph;
use srdf::{
    QuerySolution, RDFFormat, RdfDataConfig, ReaderMode, SRDFBuilder, SRDFSparql, VarName, SRDF,
};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
use std::str::FromStr;
use std::time::Instant;
use supports_color::Stream;
use tracing::debug;

pub mod cli;
pub mod input_convert_format;
pub mod input_spec;
pub mod output_convert_format;

pub use cli::{
    ShExFormat as CliShExFormat, ShaclFormat as CliShaclFormat, ShapeMapFormat as CliShapeMapFormat,
};
pub use input_convert_format::InputConvertFormat;
pub use input_spec::*;
pub use output_convert_format::OutputConvertFormat;

use shex_ast::ast::Schema as SchemaJson;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, fmt};

#[allow(unused_variables)]
fn main() -> Result<()> {
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .with_writer(io::stderr)
        .without_time();
    // Attempts to get the value of RUST_LOG which can be info, debug, trace, If unset, it uses "info"
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    // tracing::info!("rudof is running...");

    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Service {
            service,
            service_format,
            output,
            result_service_format,
            config,
            reader_mode,
            force_overwrite,
        }) => run_service(
            service,
            service_format,
            reader_mode,
            output,
            result_service_format,
            config,
            *force_overwrite,
        ),
        Some(Command::Shex {
            schema,
            schema_format,
            result_schema_format,
            output,
            show_time,
            show_statistics,
            force_overwrite,
            reader_mode,
            config,
        }) => {
            let config = get_config(config)?;
            /*if let Some(flag) = show_statistics {
                config.set_show_extends(*flag);
            }*/
            let show_time = (*show_time).unwrap_or_default();
            run_shex(
                schema,
                schema_format,
                result_schema_format,
                output,
                show_time,
                *force_overwrite,
                reader_mode,
                &config,
            )
        }
        Some(Command::Validate {
            validation_mode,
            schema,
            schema_format,
            data,
            data_format,
            reader_mode,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            max_steps,
            shacl_validation_mode,
            output,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            match validation_mode {
                ValidationMode::ShEx => run_validate_shex(
                    schema,
                    schema_format,
                    data,
                    data_format,
                    endpoint,
                    reader_mode,
                    node,
                    shape,
                    shapemap,
                    shapemap_format,
                    cli.debug,
                    output,
                    &config,
                    *force_overwrite,
                ),
                ValidationMode::SHACL => {
                    let shacl_format = match &schema_format {
                        None => Ok::<Option<cli::ShaclFormat>, anyhow::Error>(None),
                        Some(f) => {
                            let f = schema_format_to_shacl_format(f)?;
                            Ok(Some(f))
                        }
                    }?;
                    run_validate_shacl(
                        schema,
                        &shacl_format,
                        data,
                        data_format,
                        endpoint,
                        reader_mode,
                        *shacl_validation_mode,
                        cli.debug,
                        output,
                        &config,
                        *force_overwrite,
                    )
                }
            }
        }
        Some(Command::ShexValidate {
            schema,
            schema_format,
            data,
            data_format,
            reader_mode,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            output,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_validate_shex(
                schema,
                schema_format,
                data,
                data_format,
                endpoint,
                reader_mode,
                node,
                shape,
                shapemap,
                shapemap_format,
                cli.debug,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::ShaclValidate {
            shapes,
            shapes_format,
            data,
            data_format,
            reader_mode,
            endpoint,
            mode,
            output,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;
            run_validate_shacl(
                shapes,
                shapes_format,
                data,
                data_format,
                endpoint,
                reader_mode,
                *mode,
                cli.debug,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Data {
            data,
            data_format,
            reader_mode,
            output,
            result_format,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;
            run_data(
                data,
                data_format,
                cli.debug,
                output,
                result_format,
                *force_overwrite,
                reader_mode,
                &config,
            )
        }
        Some(Command::Node {
            data,
            data_format,
            endpoint,
            reader_mode,
            node,
            predicates,
            show_node_mode,
            show_hyperlinks,
            output,
            config,
            force_overwrite,
        }) => {
            let config = get_config(config)?;
            run_node(
                data,
                data_format,
                endpoint,
                reader_mode,
                node,
                predicates,
                show_node_mode,
                show_hyperlinks,
                cli.debug,
                output,
                &config,
                *force_overwrite,
            )
        }
        Some(Command::Shapemap {
            shapemap,
            shapemap_format,
            result_shapemap_format,
            output,
            force_overwrite,
        }) => run_shapemap(
            shapemap,
            shapemap_format,
            result_shapemap_format,
            output,
            *force_overwrite,
        ),
        Some(Command::Shacl {
            shapes,
            shapes_format,
            reader_mode,
            result_shapes_format,
            output,
            force_overwrite,
            config,
        }) => {
            let config = get_config(config)?;
            run_shacl(
                shapes,
                shapes_format,
                result_shapes_format,
                output,
                *force_overwrite,
                reader_mode,
                &config,
            )
        }
        Some(Command::DCTap {
            file,
            format,
            result_format,
            config,
            output,
            force_overwrite,
        }) => run_dctap(
            file,
            format,
            result_format,
            output,
            config,
            *force_overwrite,
        ),
        Some(Command::Convert {
            file,
            format,
            input_mode,
            shape,
            result_format,
            output,
            output_mode,
            target_folder,
            force_overwrite,
            config,
            reader_mode,
        }) => run_convert(
            file,
            format,
            input_mode,
            shape,
            result_format,
            output,
            output_mode,
            target_folder,
            config,
            *force_overwrite,
            reader_mode,
        ),
        Some(Command::Query {
            query,
            data,
            data_format,
            endpoint,
            reader_mode,
            output,
            result_query_format,
            config,
            force_overwrite,
        }) => {
            let query_config = get_query_config(config)?;
            run_query(
                data,
                data_format,
                endpoint,
                reader_mode,
                query,
                result_query_format,
                output,
                &query_config,
                cli.debug,
                *force_overwrite,
            )
        }
        None => {
            bail!("Command not specified")
        }
    }
}

fn run_service(
    input: &InputSpec,
    data_format: &DataFormat,
    reader_mode: &RDFReaderMode,
    output: &Option<PathBuf>,
    result_format: &ResultServiceFormat,
    config: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let config = get_config(config)?;
    let reader = input.open_read(Some(data_format.mime_type().as_str()))?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let rdf_format = data_format2rdf_format(data_format);
    let config = config.service_config();
    let base = config.base.as_ref().map(|i| i.as_str());
    let reader_mode = reader_mode_convert(*reader_mode);
    let service_description =
        ServiceDescription::from_reader(reader, &rdf_format, base, &reader_mode)?;
    match result_format {
        ResultServiceFormat::Internal => {
            writeln!(writer, "{service_description}")?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_shex(
    input: &InputSpec,
    schema_format: &CliShExFormat,
    result_schema_format: &CliShExFormat,
    output: &Option<PathBuf>,
    show_time: bool,
    force_overwrite: bool,
    _reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let begin = Instant::now();
    let (writer, color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    parse_shex_schema_rudof(&mut rudof, input, schema_format, config)?;
    show_schema_rudof(&rudof, result_schema_format, writer, color)?;
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
    Ok(())
}

// TODO: Replace by show_schema_rudof
fn show_schema(
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
        _ => Err(anyhow!(
            "Not implemented conversion to {result_schema_format} yet"
        )),
    }
}

fn show_schema_rudof(
    rudof: &Rudof,
    result_schema_format: &CliShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    if let Some(schema) = rudof.shex_schema() {
        match result_schema_format {
            CliShExFormat::Internal => {
                writeln!(writer, "{:?}", schema)?;
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
            _ => Err(anyhow!(
                "Not implemented conversion to {result_schema_format} yet"
            )),
        }
    } else {
        bail!("No ShEx schema");
    }
}

#[allow(clippy::too_many_arguments)]
fn run_validate_shex(
    schema: &Option<InputSpec>,
    schema_format: &Option<CliShExFormat>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap: &Option<InputSpec>,
    shapemap_format: &CliShapeMapFormat,
    _debug: u8,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    if let Some(schema) = schema {
        let mut rudof = Rudof::new(config);
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        let schema_format = schema_format.unwrap_or_default();
        let schema_reader = schema.open_read(Some(&schema_format.mime_type()))?;
        let schema_format = match schema_format {
            CliShExFormat::ShExC => ShExFormat::ShExC,
            CliShExFormat::ShExJ => ShExFormat::ShExJ,
            _ => bail!("ShExJ validation not yet implemented"),
        };
        let base_iri = config.shex_config().base;
        let schema_base = base_iri.as_ref().map(|iri| iri.as_str());
        rudof.read_shex(schema_reader, schema_base, &schema_format)?;
        get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;

        let shapemap_format = shapemap_format_convert(shapemap_format);
        if let Some(shapemap_spec) = shapemap {
            let shapemap_reader = shapemap_spec.open_read(None)?;
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
        writeln!(writer, "Result:\n{}", result)?;
        Ok(())
    } else {
        bail!("No ShEx schema specified")
    }
}

#[allow(clippy::too_many_arguments)]
fn run_validate_shacl(
    schema: &Option<InputSpec>,
    shapes_format: &Option<CliShaclFormat>,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    mode: ShaclValidationMode,
    _debug: u8,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;
    let result = if let Some(schema) = schema {
        let reader_mode = reader_mode_convert(*reader_mode);
        let shapes_format = shapes_format.unwrap_or_default();
        add_shacl_schema_rudof(&mut rudof, schema, &shapes_format, &reader_mode, config)?;
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_schema())
    } else {
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_data())
    }?;

    writeln!(writer, "Result:\n{}", result)?;
    Ok(())
}

fn run_shacl(
    input: &InputSpec,
    shapes_format: &CliShaclFormat,
    result_shapes_format: &CliShaclFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    let reader_mode = reader_mode_convert(*reader_mode);
    add_shacl_schema_rudof(&mut rudof, input, shapes_format, &reader_mode, config)?;
    let shacl_schema = rudof.get_shacl().unwrap();
    match result_shapes_format {
        CliShaclFormat::Internal => {
            writeln!(writer, "{shacl_schema}")?;
            Ok(())
        }
        _ => {
            let data_format = shacl_format2rdf_format(result_shapes_format);
            let mut shacl_writer: ShaclWriter<SRDFGraph> = ShaclWriter::new();
            shacl_writer.write(shacl_schema)?;
            shacl_writer.serialize(data_format, &mut writer)?;
            Ok(())
        }
    }
}

fn run_dctap(
    input: &InputSpec,
    format: &DCTapFormat,
    result_format: &DCTapResultFormat,
    output: &Option<PathBuf>,
    config: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let dctap_config = match config {
        Some(config_path) => DCTapConfig::from_path(config_path),
        None => Ok(DCTapConfig::default()),
    }?;
    let tap_config = dctap_config.dctap.unwrap_or_default();
    let dctap = parse_dctap(input, format, &tap_config)?;
    match result_format {
        DCTapResultFormat::Internal => {
            writeln!(writer, "{dctap}")?;
            Ok(())
        }
        DCTapResultFormat::JSON => {
            let str = serde_json::to_string_pretty(&dctap)
                .context("Error converting DCTap to JSON: {dctap}")?;
            writeln!(writer, "{str}")?;
            Ok(())
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_convert(
    input: &InputSpec,
    format: &InputConvertFormat,
    input_mode: &InputConvertMode,
    maybe_shape_str: &Option<String>,
    result_format: &OutputConvertFormat,
    output: &Option<PathBuf>,
    output_mode: &OutputConvertMode,
    target_folder: &Option<PathBuf>,
    config: &Option<PathBuf>,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
) -> Result<()> {
    // let mut writer = get_writer(output)?;
    let config = get_config(config)?;
    match (input_mode, output_mode) {
        (InputConvertMode::DCTAP, OutputConvertMode::ShEx) => {
            run_tap2shex(input, format, output, result_format, &config, force_overwrite)
        }
        (InputConvertMode::ShEx, OutputConvertMode::SPARQL) => {
            let maybe_shape = match maybe_shape_str {
                None => None,
                Some(shape_str) => {
                    let iri_shape = parse_iri_ref(shape_str)?;
                    Some(iri_shape)
                }
            };
            run_shex2sparql(input, format, maybe_shape, output, result_format, &config, force_overwrite, reader_mode)
        }
        (InputConvertMode::ShEx, OutputConvertMode::UML) => {
            run_shex2uml(input, format, output, result_format, maybe_shape_str, &config, force_overwrite, reader_mode)
        }
        (InputConvertMode::SHACL, OutputConvertMode::ShEx) => {
            run_shacl2shex(input, format, output, result_format, &config, force_overwrite, reader_mode)
        }
        (InputConvertMode::ShEx, OutputConvertMode::HTML) => {
            match target_folder {
                None => Err(anyhow!(
            "Conversion from ShEx to HTML requires an output parameter to indicate where to write the generated HTML files"
                )),
                Some(output_path) => {
                    run_shex2html(input, format, output_path, &config, reader_mode)
                }
            }
        }
        (InputConvertMode::DCTAP, OutputConvertMode::UML, ) => {
            run_tap2uml(input, format, output, maybe_shape_str, result_format, &config, force_overwrite)
        }
        (InputConvertMode::DCTAP, OutputConvertMode::HTML) => {
            match target_folder {
                None => Err(anyhow!(
            "Conversion from DCTAP to HTML requires an output parameter to indicate where to write the generated HTML files"
                )),
                Some(output_path) => {
                    run_tap2html(input, format, output_path, &config)
                }
            }
        }
        _ => Err(anyhow!(
            "Conversion from {input_mode} to {output_mode} is not supported yet"
        )),
    }
}

fn run_shacl2shex(
    input: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
        _ => Err(anyhow!("Can't obtain SHACL format from {format}")),
    }?;
    let mut rudof = Rudof::new(config);
    let reader_mode = reader_mode_convert(*reader_mode);
    add_shacl_schema_rudof(&mut rudof, input, &schema_format, &reader_mode, config)?;
    let shacl_schema = rudof.get_shacl().unwrap();
    let mut converter = Shacl2ShEx::new(&config.shacl2shex_config());

    converter.convert(shacl_schema)?;
    let (writer, color) = get_writer(output, force_overwrite)?;
    let result_schema_format = match &result_format {
        OutputConvertFormat::Default => CliShExFormat::ShExC,
        OutputConvertFormat::JSON => CliShExFormat::ShExJ,
        OutputConvertFormat::ShExC => CliShExFormat::ShExC,
        OutputConvertFormat::ShExJ => CliShExFormat::ShExJ,
        OutputConvertFormat::Turtle => CliShExFormat::Turtle,
        _ => {
            bail!("Shacl2ShEx converter, {result_format} format not supported for ShEx output")
        }
    };
    show_schema(
        converter.current_shex(),
        &result_schema_format,
        writer,
        color,
    )?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_shex2uml(
    input: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    maybe_shape: &Option<String>,
    config: &RudofConfig,
    force_overwrite: bool,
    _reader_mode: &RDFReaderMode,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(CliShExFormat::ShExC),
        InputConvertFormat::ShExJ => Ok(CliShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let mut rudof = Rudof::new(config);
    parse_shex_schema_rudof(&mut rudof, input, &schema_format, config)?;
    let mut converter = ShEx2Uml::new(&config.shex2uml_config());
    if let Some(schema) = rudof.shex_schema() {
        converter.convert(schema)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        generate_uml_output(converter, maybe_shape, &mut writer, result_format)?;
    } else {
        bail!("No ShEx schema")
    }
    Ok(())
}

fn generate_uml_output(
    uml_converter: ShEx2Uml,
    maybe_shape: &Option<String>,
    writer: &mut Box<dyn Write>,
    result_format: &OutputConvertFormat,
) -> Result<()> {
    let mode = if let Some(str) = maybe_shape {
        UmlGenerationMode::neighs(str)
    } else {
        UmlGenerationMode::all()
    };
    match result_format {
        OutputConvertFormat::PlantUML => {
            uml_converter.as_plantuml(writer, &mode)?;
            Ok(())
        }
        OutputConvertFormat::SVG => {
            uml_converter.as_image(writer, ImageFormat::SVG, &mode)?;
            Ok(())
        }
        OutputConvertFormat::PNG => {
            uml_converter.as_image(writer, ImageFormat::PNG, &mode)?;
            Ok(())
        }
        OutputConvertFormat::Default => {
            uml_converter.as_plantuml(writer, &mode)?;
            Ok(())
        }
        _ => Err(anyhow!(
            "Conversion to UML does not support output format {result_format}"
        )),
    }
}

fn run_shex2html<P: AsRef<Path>>(
    input: &InputSpec,
    format: &InputConvertFormat,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    config: &RudofConfig,
    _reader_mode: &RDFReaderMode,
) -> Result<()> {
    debug!("Starting shex2html");
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(CliShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let mut rudof = Rudof::new(config);

    parse_shex_schema_rudof(&mut rudof, input, &schema_format, config)?;
    if let Some(schema) = rudof.shex_schema() {
        let shex2html_config = config.shex2html_config();
        let config = shex2html_config
            .clone()
            .with_target_folder(output_folder.as_ref());
        let landing_page = config.landing_page().to_string_lossy().to_string();
        debug!("Landing page will be generated at {landing_page}\nStarted converter...");
        let mut converter = ShEx2Html::new(config);
        converter.convert(schema)?;
        converter.export_schema()?;
        debug!("HTML pages generated at {}", landing_page);
    } else {
        bail!("No ShEx schema")
    }
    Ok(())
}

fn run_tap2html<P: AsRef<Path>>(
    input: &InputSpec,
    format: &InputConvertFormat,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    config: &RudofConfig,
) -> Result<()> {
    debug!("Starting tap2html");
    let dctap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(DCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input, &dctap_format, &config.tap_config())?;
    let converter_tap = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter_tap.convert(&dctap)?;
    debug!(
        "Converted ShEx: {}",
        ShExFormatter::default().format_schema(&shex)
    );
    let shex2html_config = config
        .shex2html_config()
        .clone()
        .with_target_folder(output_folder.as_ref());
    let landing_page = shex2html_config
        .landing_page()
        .to_string_lossy()
        .to_string();
    debug!("Landing page {landing_page}\nConverter...");
    let mut converter = ShEx2Html::new(shex2html_config);
    converter.convert(&shex)?;
    // debug!("Converted HTMLSchema: {:?}", converter.current_html());
    converter.export_schema()?;
    debug!("HTML pages generated at {}", landing_page);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_shex2sparql(
    input: &InputSpec,
    format: &InputConvertFormat,
    shape: Option<IriRef>,
    output: &Option<PathBuf>,
    _result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
    _reader_mode: &RDFReaderMode,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(CliShExFormat::ShExC),
        InputConvertFormat::ShExJ => Ok(CliShExFormat::ShExJ),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let mut rudof = Rudof::new(config);
    parse_shex_schema_rudof(&mut rudof, input, &schema_format, config)?;
    if let Some(schema) = rudof.shex_schema() {
        let converter = ShEx2Sparql::new(&config.shex2sparql_config());
        let sparql = converter.convert(schema, shape)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        write!(writer, "{}", sparql)?;
    }
    Ok(())
}

fn run_tap2shex(
    input_path: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(DCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input_path, &tap_format, &config.tap_config())?;
    let converter = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter.convert(&dctap)?;
    let result_schema_format = match result_format {
        OutputConvertFormat::Default => Ok(CliShExFormat::ShExC),
        OutputConvertFormat::Internal => Ok(CliShExFormat::Internal),
        OutputConvertFormat::ShExJ => Ok(CliShExFormat::ShExJ),
        OutputConvertFormat::Turtle => Ok(CliShExFormat::Turtle),
        _ => Err(anyhow!("Can't write ShEx in {result_format} format")),
    }?;
    let (writer, color) = get_writer(output, force_overwrite)?;
    show_schema(&shex, &result_schema_format, writer, color)?;
    Ok(())
}

fn run_tap2uml(
    input_path: &InputSpec,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    maybe_shape: &Option<String>,
    result_format: &OutputConvertFormat,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(DCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input_path, &tap_format, &config.tap_config())?;
    let converter_shex = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter_shex.convert(&dctap)?;
    let mut converter_uml = ShEx2Uml::new(&config.shex2uml_config());
    converter_uml.convert(&shex)?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    generate_uml_output(converter_uml, maybe_shape, &mut writer, result_format)?;
    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum ColorSupport {
    NoColor,
    WithColor,
}

fn get_writer(
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<(Box<dyn Write>, ColorSupport)> {
    match output {
        None => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            let color_support = match supports_color::on(Stream::Stdout) {
                Some(_) => ColorSupport::WithColor,
                _ => ColorSupport::NoColor,
            };
            Ok((Box::new(handle), color_support))
        }
        Some(path) => {
            let file = if Path::exists(path) {
                if force_overwrite {
                    OpenOptions::new().write(true).truncate(true).open(path)
                } else {
                    bail!("File {} already exists. If you want to overwrite it, use the `force-overwrite` option", path.display());
                }
            } else {
                File::create(path)
            }?;
            let writer = BufWriter::new(file);
            Ok((Box::new(writer), ColorSupport::NoColor))
        }
    }
}

fn get_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    _debug: u8,
    config: &RdfDataConfig,
) -> Result<RdfData> {
    match (data.is_empty(), endpoint) {
        (true, None) => {
            bail!("None of `data` or `endpoint` parameters have been specified for validation")
        }
        (false, None) => {
            // let data_path = cast_to_data_path(data)?;
            let data = parse_data(data, data_format, reader_mode, config)?;
            Ok(RdfData::from_graph(data)?)
        }
        (true, Some(endpoint)) => {
            let endpoint = SRDFSparql::from_str(endpoint)?;
            Ok(RdfData::from_endpoint(endpoint))
        }
        (false, Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' supported at the same time at this moment")
        }
    }
}

fn add_shacl_schema_rudof(
    rudof: &mut Rudof,
    schema: &InputSpec,
    shapes_format: &CliShaclFormat,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let reader = schema.open_read(Some(shapes_format.mime_type().as_str()))?;
    let shapes_format = shacl_format_convert(shapes_format)?;
    let base = config.rdf_data_base();
    rudof.read_shacl(reader, &shapes_format, base, reader_mode)?;
    Ok(())
}

fn get_data_rudof(
    rudof: &mut Rudof,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let base = config.rdf_data_base();
    match (data.is_empty(), endpoint) {
        (true, None) => {
            bail!("None of `data` or `endpoint` parameters have been specified for validation")
        }
        (false, None) => {
            let rdf_format = data_format2rdf_format(data_format);
            let reader_mode = match &reader_mode {
                RDFReaderMode::Lax => srdf::ReaderMode::Lax,
                RDFReaderMode::Strict => srdf::ReaderMode::Strict,
            };
            for d in data {
                let data_reader = d.open_read(Some(&data_format.mime_type()))?;
                rudof.read_data(data_reader, &rdf_format, base, &reader_mode)?;
            }
            Ok(())
        }
        (true, Some(endpoint)) => {
            let endpoint_iri = IriS::from_str(endpoint.as_str())?;
            rudof.add_endpoint(&endpoint_iri)?;
            Ok(())
        }
        (false, Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' supported at the same time at this moment")
        }
    }
}

fn get_query_str(input: &InputSpec) -> Result<String> {
    let mut str = String::new();
    let mut data = input.open_read(None)?;
    data.read_to_string(&mut str)?;
    Ok(str)
}

/*fn make_node_selector(node: Node) -> Result<NodeSelector> {
    let object = node.as_object();
    match object {
        Object::Iri { iri } => Ok(NodeSelector::Node(ObjectValue::iri(iri.clone()))),
        Object::BlankNode(_) => bail!("Blank nodes can not be used as node selectors to validate"),
        Object::Literal(lit) => Ok(NodeSelector::Node(ObjectValue::Literal(lit.clone()))),
    }
}

fn make_shape_selector(shape_label: ShapeExprLabel) -> ShapeSelector {
    ShapeSelector::Label(shape_label)
}
*/

fn start() -> ShapeSelector {
    ShapeSelector::start()
}

#[allow(clippy::too_many_arguments)]
fn run_node(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    node_str: &str,
    predicates: &Vec<String>,
    show_node_mode: &ShowNodeMode,
    show_hyperlinks: &bool,
    _debug: u8,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;
    let data = rudof.rdf_data();
    let node_selector = parse_node_selector(node_str)?;
    show_node_info(
        node_selector,
        predicates,
        data,
        show_node_mode,
        show_hyperlinks,
        &mut writer,
    )
}

fn show_node_info<S, W: Write>(
    node_selector: NodeSelector,
    predicates: &Vec<String>,
    rdf: &S,
    show_node_mode: &ShowNodeMode,
    _show_hyperlinks: &bool,
    writer: &mut W,
) -> Result<()>
where
    S: SRDF,
{
    for node in node_selector.iter_node(rdf) {
        let subject = node_to_subject(node, rdf)?;
        writeln!(writer, "Information about node")?;

        // Show outgoing arcs
        match show_node_mode {
            ShowNodeMode::Outgoing | ShowNodeMode::Both => {
                writeln!(writer, "Outgoing arcs")?;
                let map = if predicates.is_empty() {
                    match rdf.outgoing_arcs(&subject) {
                        Result::Ok(rs) => rs,
                        Err(e) => bail!("Error obtaining outgoing arcs of {subject}: {e}"),
                    }
                } else {
                    let preds = cnv_predicates(predicates, rdf)?;
                    match rdf.outgoing_arcs_from_list(&subject, &preds) {
                        Result::Ok((rs, _)) => rs,
                        Err(e) => bail!("Error obtaining outgoing arcs of {subject}: {e}"),
                    }
                };
                writeln!(writer, "{}", rdf.qualify_subject(&subject))?;
                for pred in map.keys() {
                    writeln!(writer, " -{}-> ", rdf.qualify_iri(pred))?;
                    if let Some(objs) = map.get(pred) {
                        for o in objs {
                            writeln!(writer, "      {}", rdf.qualify_term(o))?;
                        }
                    } else {
                        bail!("Not found values for {pred} in map")
                    }
                }
            }
            _ => {
                // Nothing to do
            }
        }

        // Show incoming arcs
        match show_node_mode {
            ShowNodeMode::Incoming | ShowNodeMode::Both => {
                writeln!(writer, "Incoming arcs")?;
                let object = S::subject_as_term(&subject);
                let map = match rdf.incoming_arcs(&object) {
                    Result::Ok(m) => m,
                    Err(e) => bail!("Can't get outgoing arcs of node {subject}: {e}"),
                };
                writeln!(writer, "{}", rdf.qualify_term(&object))?;
                for pred in map.keys() {
                    writeln!(writer, "  <-{}-", rdf.qualify_iri(pred))?;
                    if let Some(subjs) = map.get(pred) {
                        for s in subjs {
                            writeln!(writer, "      {}", rdf.qualify_subject(s))?;
                        }
                    } else {
                        bail!("Not found values for {pred} in map")
                    }
                }
            }
            _ => {
                // Nothing to do
            }
        }
    }
    Ok(())
}

fn cnv_predicates<S>(predicates: &Vec<String>, rdf: &S) -> Result<Vec<S::IRI>>
where
    S: SRDF,
{
    let mut vs = Vec::new();
    for s in predicates {
        let iri_ref = parse_iri_ref(s)?;
        let iri_s = match iri_ref {
            IriRef::Prefixed { prefix, local } => {
                rdf.resolve_prefix_local(prefix.as_str(), local.as_str())?
            }
            IriRef::Iri(iri) => iri,
        };
        let iri = S::iri_s2iri(&iri_s);
        vs.push(iri)
    }
    Ok(vs)
}

fn run_shapemap(
    shapemap: &InputSpec,
    shapemap_format: &CliShapeMapFormat,
    result_format: &CliShapeMapFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(&RudofConfig::new());
    let shapemap_format = shapemap_format_convert(shapemap_format);
    rudof.read_shapemap(shapemap.open_read(None)?, &shapemap_format)?;
    let shapemap = rudof.get_shapemap().unwrap();
    match result_format {
        CliShapeMapFormat::Compact => {
            let str = ShapemapFormatter::default().format_shapemap(shapemap);
            writeln!(writer, "{str}")?;
            Ok(())
        }
        CliShapeMapFormat::Internal => {
            writeln!(writer, "{shapemap:?}")?;
            Ok(())
        }
    }
}

fn node_to_subject<S>(node: &ObjectValue, rdf: &S) -> Result<S::Subject>
where
    S: SRDF,
{
    match node {
        ObjectValue::IriRef(iri_ref) => {
            let iri = match iri_ref {
                IriRef::Iri(iri_s) => S::iri_s2iri(iri_s),
                IriRef::Prefixed { prefix, local } => {
                    let iri_s = rdf.resolve_prefix_local(prefix, local)?;

                    S::iri_s2iri(&iri_s)
                }
            };
            let term = S::iri_as_term(iri);
            match S::term_as_subject(&term) {
                None => bail!("node_to_subject: Can't convert term {term} to subject"),
                Some(subject) => Ok(subject),
            }
        }
        ObjectValue::Literal(_lit) => Err(anyhow!("Node must be an IRI")),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    _debug: u8,
    output: &Option<PathBuf>,
    result_format: &DataFormat,
    force_overwrite: bool,
    reader_mode: &RDFReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(&mut rudof, data, data_format, &None, reader_mode, config)?;
    rudof
        .rdf_data()
        .serialize(RDFFormat::from(*result_format), &mut writer)?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_query(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    reader_mode: &RDFReaderMode,
    query: &InputSpec,
    _result_query_format: &ResultQueryFormat,
    output: &Option<PathBuf>,
    config: &QueryConfig,
    debug: u8,
    force_overwrite: bool,
) -> Result<()> {
    use crate::srdf::QuerySRDF;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let data_config = match &config.data_config {
        None => RdfDataConfig::default(),
        Some(dc) => dc.clone(),
    };
    let data = get_data(
        data,
        data_format,
        endpoint,
        reader_mode,
        debug,
        &data_config,
    )?;
    let query = get_query_str(query)?;
    let results = data.query_select(query.as_str())?;
    let mut results_iter = results.iter().peekable();
    if let Some(first) = results_iter.peek() {
        show_variables(&mut writer, first.variables())?;
        for result in results_iter {
            show_result(&mut writer, result, &data.prefixmap_in_memory())?
        }
    } else {
        write!(writer, "No results")?;
    }
    Ok(())
}

fn show_variables<'a, W: Write>(
    writer: &mut W,
    vars: impl Iterator<Item = &'a VarName>,
) -> Result<()> {
    for var in vars {
        let str = format!("{}", var);
        write!(writer, "{:15}", str)?;
    }
    writeln!(writer)?;
    Ok(())
}

fn show_result<W: Write>(
    writer: &mut W,
    result: &QuerySolution<RdfData>,
    prefixmap: &PrefixMap,
) -> Result<()> {
    for (idx, _variable) in result.variables().enumerate() {
        let str = match result.find_solution(idx) {
            Some(term) => match term {
                oxrdf::Term::NamedNode(named_node) => {
                    let (str, length) =
                        prefixmap.qualify_and_length(&IriS::from_named_node(named_node));
                    format!("{}{}", " ".repeat(15 - length), str)
                }
                oxrdf::Term::BlankNode(blank_node) => format!("  {}", blank_node),
                oxrdf::Term::Literal(literal) => format!("  {}", literal),
                oxrdf::Term::Triple(triple) => format!("  {}", triple),
            },
            None => String::new(),
        };
        write!(writer, "{:15}", str)?;
    }
    writeln!(writer)?;
    Ok(())
}

fn parse_shex_schema_rudof(
    rudof: &mut Rudof,
    input: &InputSpec,
    schema_format: &CliShExFormat,
    config: &RudofConfig,
) -> Result<()> {
    let reader = input
        .open_read(Some(&schema_format.mime_type()))
        .context(format!("Get reader from input: {input}"))?;
    let schema_format = shex_format_convert(schema_format);
    let shex_config = config.shex_config();
    let base = base_convert(&shex_config.base);
    rudof.read_shex(reader, base, &schema_format)?;
    Ok(())
}

/*fn parse_shacl_rudof(
    rudof: &mut Rudof,
    input: &InputSpec,
    shapes_format: &CliShaclFormat,
    reader_mode: &RDFReaderMode,
    config: &RdfDataConfig,
) -> Result<ShaclSchema> {
    let reader = input.open_read(Some(&shapes_format.mime_type()))?;
    let base = config.base.as_ref().map(|i| i.as_str());
    let shacl_format = shacl_format_convert(shapes_format)?;
    let reader_mode = reader_mode_convert(*reader_mode);
    // rudof.read_shacl(reader, base, &shacl_format, &reader_mode)?;
    rudof.merge_data_from_reader(reader, , base, reader_mode)?;
    if let Some(shacl_schema) = rudof.shacl_schema() {
        Ok(shacl_schema.clone())
    } else {
        bail!("Internal error: SHACL schema was read but it not returned by rudof");
    }
}*/

/*
fn parse_shacl(
    input: &InputSpec,
    shapes_format: &ShaclFormat,
    reader_mode: &RDFReaderMode,
    config: &RdfDataConfig,
) -> Result<ShaclSchema> {
    match shapes_format {
        ShaclFormat::Internal => bail!("Cannot read internal ShEx format yet"),
        _ => {
            let data_format = shacl_format_to_data_format(shapes_format)?;
            let rdf = parse_data(&vec![input.clone()], &data_format, reader_mode, config)?;
            let schema = ShaclParser::new(rdf).parse()?;
            Ok(schema)
        }
    }
} */

fn parse_dctap(input: &InputSpec, format: &DCTapFormat, config: &TapConfig) -> Result<DCTap> {
    match format {
        DCTapFormat::CSV => {
            let reader = input.open_read(None)?;
            let dctap = DCTap::from_reader(reader, config)?;
            Ok(dctap)
        }
        DCTapFormat::XLS | DCTapFormat::XLSB | DCTapFormat::XLSM | DCTapFormat::XLSX => match input
        {
            InputSpec::Path(path_buf) => {
                let dctap = DCTap::from_excel(path_buf, None, config)?;
                Ok(dctap)
            }
            InputSpec::Stdin => bail!("Can not read Excel file from stdin"),
            InputSpec::Url(_) => bail!("Not implemented reading Excel files from URIs yet"),
        },
    }
}

fn shacl_format_convert(shacl_format: &cli::ShaclFormat) -> Result<ShaclFormat> {
    match shacl_format {
        cli::ShaclFormat::Turtle => Ok(ShaclFormat::Turtle),
        cli::ShaclFormat::RDFXML => Ok(ShaclFormat::RDFXML),
        cli::ShaclFormat::NTriples => Ok(ShaclFormat::NTriples),
        cli::ShaclFormat::TriG => Ok(ShaclFormat::TriG),
        cli::ShaclFormat::N3 => Ok(ShaclFormat::N3),
        cli::ShaclFormat::NQuads => Ok(ShaclFormat::NQuads),
        cli::ShaclFormat::Internal => {
            bail!("Cannot convert internal SHACL format to RDF data format")
        }
    }
}

fn data_format2rdf_format(data_format: &DataFormat) -> RDFFormat {
    match data_format {
        DataFormat::N3 => RDFFormat::N3,
        DataFormat::NQuads => RDFFormat::NQuads,
        DataFormat::NTriples => RDFFormat::NTriples,
        DataFormat::RDFXML => RDFFormat::RDFXML,
        DataFormat::TriG => RDFFormat::TriG,
        DataFormat::Turtle => RDFFormat::Turtle,
    }
}

fn shacl_format2rdf_format(data_format: &CliShaclFormat) -> RDFFormat {
    match data_format {
        CliShaclFormat::N3 => RDFFormat::N3,
        CliShaclFormat::NQuads => RDFFormat::NQuads,
        CliShaclFormat::NTriples => RDFFormat::NTriples,
        CliShaclFormat::RDFXML => RDFFormat::RDFXML,
        CliShaclFormat::TriG => RDFFormat::TriG,
        CliShaclFormat::Turtle => RDFFormat::Turtle,
        CliShaclFormat::Internal => todo!(),
    }
}

fn parse_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    reader_mode: &RDFReaderMode,
    config: &RdfDataConfig,
) -> Result<SRDFGraph> {
    let mut graph = SRDFGraph::new();
    let rdf_format = data_format2rdf_format(data_format);
    for d in data {
        let reader = d.open_read(Some(data_format.mime_type().as_str()))?;
        let base = config.base.as_ref().map(|iri_s| iri_s.as_str());
        let reader_mode = reader_mode_convert(*reader_mode);
        graph.merge_from_reader(reader, &rdf_format, base, &reader_mode)?;
    }
    Ok(graph)
}

fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let ns = ShapeMapParser::parse_node_selector(node_str)?;
    Ok(ns)
}

fn parse_shape_selector(label_str: &str) -> Result<ShapeSelector> {
    let selector = ShapeMapParser::parse_shape_selector(label_str)?;
    Ok(selector)
}

fn parse_iri_ref(iri: &str) -> Result<IriRef> {
    let iri = ShapeMapParser::parse_iri_ref(iri)?;
    Ok(iri)
}

fn get_config(config: &Option<PathBuf>) -> Result<RudofConfig> {
    match config {
        Some(config_path) => match RudofConfig::from_path(config_path) {
            Ok(c) => Ok(c),
            Err(e) => Err(anyhow!(
                "Error obtaining Data config from {}: {e}",
                config_path.display()
            )),
        },
        None => Ok(RudofConfig::default()),
    }
}

fn get_query_config(config: &Option<PathBuf>) -> Result<QueryConfig> {
    match config {
        Some(config_path) => match QueryConfig::from_path(config_path) {
            Ok(c) => Ok(c),
            Err(e) => Err(anyhow!(
                "Error obtaining Query config from {}: {e}",
                config_path.display()
            )),
        },
        None => Ok(QueryConfig::default()),
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

fn shapemap_format_convert(shapemap_format: &CliShapeMapFormat) -> ShapemapFormat {
    match shapemap_format {
        CliShapeMapFormat::Compact => ShapemapFormat::Compact,
        CliShapeMapFormat::Internal => ShapemapFormat::JSON,
    }
}

fn shex_format_convert(shex_format: &CliShExFormat) -> ShExFormat {
    match shex_format {
        CliShExFormat::ShExC => ShExFormat::ShExC,
        CliShExFormat::ShExJ => ShExFormat::ShExJ,
        CliShExFormat::Turtle => ShExFormat::Turtle,
        _ => ShExFormat::ShExC,
    }
}

fn base_convert(base: &Option<IriS>) -> Option<&str> {
    base.as_ref().map(|iri| iri.as_str())
}

fn reader_mode_convert(rm: RDFReaderMode) -> ReaderMode {
    rm.into()
}

fn schema_format_to_shacl_format(f: &CliShExFormat) -> Result<CliShaclFormat> {
    match f {
        CliShExFormat::Internal => Ok(CliShaclFormat::Internal),
        CliShExFormat::ShExC => Err(anyhow!(
            "Validation using SHACL mode doesn't support ShExC format"
        )),
        CliShExFormat::Simple => Err(anyhow!(
            "Validation using SHACL mode doesn't support {f} format"
        )),
        CliShExFormat::ShExJ => bail!("Validation using SHACL mode doesn't support ShExC format"),
        CliShExFormat::Turtle => Ok(CliShaclFormat::Turtle),
        CliShExFormat::NTriples => Ok(CliShaclFormat::NTriples),
        CliShExFormat::RDFXML => Ok(CliShaclFormat::RDFXML),
        CliShExFormat::TriG => Ok(CliShaclFormat::TriG),
        CliShExFormat::N3 => Ok(CliShaclFormat::N3),
        CliShExFormat::NQuads => Ok(CliShaclFormat::NQuads),
    }
}
