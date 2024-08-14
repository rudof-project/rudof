extern crate anyhow;
extern crate clap;
extern crate dctap;
extern crate iri_s;
extern crate oxrdf;
extern crate prefixmap;
extern crate regex;
extern crate serde_json;
extern crate shacl_ast;
extern crate shacl_validation;
extern crate shapemap;
extern crate shapes_converter;
extern crate shex_ast;
extern crate shex_compact;
extern crate shex_validation;
extern crate srdf;
extern crate supports_color;
extern crate tracing;
extern crate tracing_subscriber;

use anyhow::*;
use clap::Parser;
use dctap::{DCTap, TapConfig};
use prefixmap::IriRef;
use shacl_ast::{Schema as ShaclSchema, ShaclParser, ShaclWriter};
use shacl_validation::validate::{GraphValidator, ShaclValidationMode, SparqlValidator};
use shapemap::{query_shape_map::QueryShapeMap, NodeSelector, ShapeSelector};
use shapes_converter::{shex_to_sparql::ShEx2SparqlConfig, ShEx2Sparql};
use shapes_converter::{
    ConverterConfig, ImageFormat, ShEx2Html, ShEx2HtmlConfig, ShEx2Uml, ShEx2UmlConfig, Tap2ShEx,
};
use shex_ast::{object_value::ObjectValue, shexr::shexr_parser::ShExRParser};
use shex_compact::{ShExFormatter, ShExParser, ShapeMapParser, ShapemapFormatter};
use shex_validation::{Validator, ValidatorConfig};
use srdf::srdf_graph::SRDFGraph;
use srdf::{RDFFormat, SRDFBuilder, SRDFSparql, SRDF};
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
use std::str::FromStr;
use std::time::Instant;
use supports_color::Stream;
use tracing::debug;

pub mod cli;
pub mod data;
pub mod input_spec;

pub use cli::*;
pub use data::*;
pub use input_spec::*;

use shex_ast::{ast::Schema as SchemaJson, compiled::compiled_schema::CompiledSchema};
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
        Some(Command::Shex {
            schema,
            schema_format,
            result_schema_format,
            output,
            show_time,
            show_statistics,
            force_overwrite,
        }) => run_shex(
            schema,
            schema_format,
            result_schema_format,
            output,
            *show_time,
            *show_statistics,
            *force_overwrite,
        ),
        Some(Command::Validate {
            validation_mode,
            schema,
            schema_format,
            data,
            data_format,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            max_steps,
            shacl_validation_mode,
            output,
            force_overwrite,
        }) => match validation_mode {
            ValidationMode::ShEx => run_validate_shex(
                schema,
                schema_format,
                data,
                data_format,
                endpoint,
                node,
                shape,
                shapemap,
                shapemap_format,
                cli.debug,
                output,
                &ValidatorConfig::default(),
                *force_overwrite,
            ),
            ValidationMode::SHACL => {
                let shacl_format = match schema_format {
                    ShExFormat::Internal => Ok(ShaclFormat::Internal),
                    ShExFormat::ShExC => Err(anyhow!(
                        "Validation using SHACL mode doesn't support ShExC format"
                    )),
                    ShExFormat::ShExJ => Err(anyhow!(
                        "Validation using SHACL mode doesn't support ShExC format"
                    )),
                    ShExFormat::Turtle => Ok(ShaclFormat::Turtle),
                    ShExFormat::NTriples => Ok(ShaclFormat::NTriples),
                    ShExFormat::RDFXML => Ok(ShaclFormat::RDFXML),
                    ShExFormat::TriG => Ok(ShaclFormat::TriG),
                    ShExFormat::N3 => Ok(ShaclFormat::N3),
                    ShExFormat::NQuads => Ok(ShaclFormat::NQuads),
                }?;
                run_validate_shacl(
                    schema,
                    &shacl_format,
                    data,
                    data_format,
                    endpoint,
                    *shacl_validation_mode,
                    cli.debug,
                    output,
                    *force_overwrite,
                )
            }
        },
        Some(Command::ShexValidate {
            schema,
            schema_format,
            data,
            data_format,
            endpoint,
            node,
            shape,
            shapemap,
            shapemap_format,
            output,
            config,
            force_overwrite,
        }) => {
            let config = match config {
                Some(config_path) => match ValidatorConfig::from_path(config_path) {
                    Ok(c) => Ok(c),
                    Err(e) => Err(anyhow!(
                        "Error obtaining ShEx validation confir from {}: {e}",
                        config_path.display()
                    )),
                },
                None => Ok(ValidatorConfig::default()),
            }?;
            run_validate_shex(
                schema,
                schema_format,
                data,
                data_format,
                endpoint,
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
            endpoint,
            mode,
            output,
            force_overwrite,
        }) => run_validate_shacl(
            shapes,
            shapes_format,
            data,
            data_format,
            endpoint,
            *mode,
            cli.debug,
            output,
            *force_overwrite,
        ),
        Some(Command::Data {
            data,
            data_format,
            output,
            result_format,
            force_overwrite,
        }) => run_data(
            data,
            data_format,
            cli.debug,
            output,
            result_format,
            *force_overwrite,
        ),
        Some(Command::Node {
            data,
            data_format,
            endpoint,
            node,
            predicates,
            show_node_mode,
            show_hyperlinks,
            output,
            config,
            force_overwrite,
        }) => run_node(
            data,
            data_format,
            endpoint,
            node,
            predicates,
            show_node_mode,
            show_hyperlinks,
            cli.debug,
            output,
            config,
            *force_overwrite,
        ),
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
            result_shapes_format,
            output,
            force_overwrite,
        }) => run_shacl(
            shapes,
            shapes_format,
            result_shapes_format,
            output,
            *force_overwrite,
        ),
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
        ),
        None => {
            println!("Command not specified");
            Ok(())
        }
    }
}

fn run_shex(
    schema_path: &Path,
    schema_format: &ShExFormat,
    result_schema_format: &ShExFormat,
    output: &Option<PathBuf>,
    show_time: bool,
    show_statistics: bool,
    force_overwrite: bool,
) -> Result<()> {
    let begin = Instant::now();
    let (writer, color) = get_writer(output, force_overwrite)?;
    let schema_json = parse_schema(schema_path, schema_format)?;
    show_schema(&schema_json, result_schema_format, writer, color)?;
    if show_time {
        let elapsed = begin.elapsed();
        let _ = writeln!(io::stderr(), "elapsed: {:.03?} sec", elapsed.as_secs_f64());
    }
    if show_statistics {
        if let Some(shapes) = schema_json.shapes() {
            let _ = writeln!(io::stderr(), "Shapes: {:?}", shapes.len());
            let _ = writeln!(io::stderr(), "Shapes extends: {:?}", schema_json);
        }
        let _ = writeln!(io::stderr(), "No shape declaration");
    }
    Ok(())
}

fn show_schema(
    schema: &SchemaJson,
    result_schema_format: &ShExFormat,
    mut writer: Box<dyn Write>,
    color: ColorSupport,
) -> Result<()> {
    match result_schema_format {
        ShExFormat::Internal => {
            writeln!(writer, "{schema:?}")?;
            Ok(())
        }
        ShExFormat::ShExC => {
            let formatter = match color {
                ColorSupport::NoColor => ShExFormatter::default().without_colors(),
                ColorSupport::WithColor => ShExFormatter::default(),
            };
            let str = formatter.format_schema(schema);
            writeln!(writer, "{str}")?;
            Ok(())
        }
        ShExFormat::ShExJ => {
            let str = serde_json::to_string_pretty(&schema)?;
            writeln!(writer, "{str}")?;
            Ok(())
        }
        _ => Err(anyhow!(
            "Not implemented conversion to {result_schema_format} yet"
        )),
    }
}

#[allow(clippy::too_many_arguments)]
fn run_validate_shex(
    schema_path: &Path,
    schema_format: &ShExFormat,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap_path: &Option<PathBuf>,
    shapemap_format: &ShapeMapFormat,
    debug: u8,
    output: &Option<PathBuf>,
    config: &ValidatorConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let schema_json = parse_schema(schema_path, schema_format)?;
    let mut schema: CompiledSchema = CompiledSchema::new();
    schema.from_schema_json(&schema_json)?;
    let data = get_data(data, data_format, endpoint, debug)?;
    let mut shapemap = match shapemap_path {
        None => QueryShapeMap::new(),
        Some(shapemap_buf) => parse_shapemap(shapemap_buf, shapemap_format)?,
    };
    match (maybe_node, maybe_shape) {
        (None, None) => {
            // Nothing to do in this case
        }
        (Some(node_str), None) => {
            let node_selector = parse_node_selector(node_str)?;
            shapemap.add_association(node_selector, start())
        }
        (Some(node_str), Some(shape_str)) => {
            let node_selector = parse_node_selector(node_str)?;
            let shape_selector = parse_shape_label(shape_str)?;
            shapemap.add_association(node_selector, shape_selector)
        }
        (None, Some(shape_str)) => {
            tracing::debug!(
                "Shape label {shape_str} ignored because noshapemap has also been provided"
            )
        }
    };
    let mut validator = Validator::new(schema, config);
    let result = match &data {
        Data::Endpoint(endpoint) => validator.validate_shapemap(&shapemap, endpoint),
        Data::RDFData(data) => validator.validate_shapemap(&shapemap, data),
    };
    match result {
        Result::Ok(_t) => match validator.result_map(data.prefixmap()) {
            Result::Ok(result_map) => {
                writeln!(writer, "Result:\n{}", result_map)?;
                Ok(())
            }
            Err(err) => {
                println!("Error generating result_map after validation: {err}");
                bail!("{err}");
            }
        },
        Result::Err(err) => {
            bail!("{err}");
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_validate_shacl(
    shapes_path: &Path,
    shapes_format: &ShaclFormat,
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    mode: ShaclValidationMode,
    _debug: u8,
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;

    // TODO: Remove the following cast by refactoring the validate_shex to support more types of data
    let data = cast_to_data_path(data)?;

    if let Some(data) = data {
        let validator = match GraphValidator::new(
            &data,
            match data_format {
                DataFormat::Turtle => srdf::RDFFormat::Turtle,
                DataFormat::NTriples => srdf::RDFFormat::NTriples,
                DataFormat::RDFXML => srdf::RDFFormat::RDFXML,
                DataFormat::TriG => srdf::RDFFormat::TriG,
                DataFormat::N3 => srdf::RDFFormat::N3,
                DataFormat::NQuads => srdf::RDFFormat::NQuads,
            },
            None,
            mode,
        ) {
            Ok(validator) => validator,
            Err(e) => bail!("Error during the creation of the Graph: {e}"),
        };
        let result = match shacl_validation::validate::Validator::validate(
            &validator,
            shapes_path,
            match shapes_format {
                ShaclFormat::Internal => todo!(),
                ShaclFormat::Turtle => srdf::RDFFormat::Turtle,
                ShaclFormat::NTriples => srdf::RDFFormat::NTriples,
                ShaclFormat::RDFXML => srdf::RDFFormat::RDFXML,
                ShaclFormat::TriG => srdf::RDFFormat::TriG,
                ShaclFormat::N3 => srdf::RDFFormat::N3,
                ShaclFormat::NQuads => srdf::RDFFormat::NQuads,
            },
        ) {
            Ok(result) => result,
            Err(e) => bail!("Error validating the graph: {e}"),
        };
        writeln!(writer, "Result:\n{}", result)?;
        Ok(())
    } else if let Some(endpoint) = endpoint {
        let validator = match SparqlValidator::new(endpoint, mode) {
            Ok(validator) => validator,
            Err(e) => bail!("Error during the creation of the Graph: {e}"),
        };
        let result = match shacl_validation::validate::Validator::validate(
            &validator,
            shapes_path,
            match shapes_format {
                ShaclFormat::Internal => todo!(),
                ShaclFormat::Turtle => srdf::RDFFormat::Turtle,
                ShaclFormat::NTriples => srdf::RDFFormat::NTriples,
                ShaclFormat::RDFXML => srdf::RDFFormat::RDFXML,
                ShaclFormat::TriG => srdf::RDFFormat::TriG,
                ShaclFormat::N3 => srdf::RDFFormat::N3,
                ShaclFormat::NQuads => srdf::RDFFormat::NQuads,
            },
        ) {
            Ok(result) => result,
            Err(e) => bail!("Error validating the graph: {e}"),
        };
        writeln!(writer, "Result:\n{}", result)?;
        Ok(())
    } else {
        bail!("Please provide either a local data source or an endpoint")
    }
}

fn run_shacl(
    shapes_path: &Path,
    shapes_format: &ShaclFormat,
    result_shapes_format: &ShaclFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let shacl_schema = parse_shacl(shapes_path, shapes_format)?;
    match result_shapes_format {
        ShaclFormat::Internal => {
            writeln!(writer, "{shacl_schema}")?;
            Ok(())
        }
        _ => {
            let data_format = shacl_format_to_data_format(result_shapes_format)?;
            let mut shacl_writer: ShaclWriter<SRDFGraph> = ShaclWriter::new();
            shacl_writer.write(&shacl_schema)?;
            shacl_writer.serialize(data_format.into(), writer)?;
            Ok(())
        }
    }
}

fn run_dctap(
    input_path: &Path,
    format: &DCTapFormat,
    result_format: &DCTapResultFormat,
    output: &Option<PathBuf>,
    config: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let tap_config = match config {
        Some(config_path) => TapConfig::from_path(config_path),
        None => Ok(TapConfig::default()),
    }?;
    let dctap = parse_dctap(input_path, format, &tap_config)?;
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
    input_path: &Path,
    format: &InputConvertFormat,
    input_mode: &InputConvertMode,
    maybe_shape_str: &Option<String>,
    result_format: &OutputConvertFormat,
    output: &Option<PathBuf>,
    output_mode: &OutputConvertMode,
    target_folder: &Option<PathBuf>,
    config: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    // let mut writer = get_writer(output)?;
    let converter_config = match config {
        None => Ok(ConverterConfig::default()),
        Some(config_path) => ConverterConfig::from_path(config_path),
    }?;
    match (input_mode, output_mode) {
        (InputConvertMode::DCTAP, OutputConvertMode::ShEx) => {
            run_tap2shex(input_path, format, output, result_format, &converter_config, force_overwrite)
        }
        (InputConvertMode::ShEx, OutputConvertMode::SPARQL) => {
            let maybe_shape = match maybe_shape_str {
                None => None,
                Some(shape_str) => {
                    let iri_shape = parse_iri_ref(shape_str)?;
                    Some(iri_shape)
                }
            };
            run_shex2sparql(input_path, format, maybe_shape, output, result_format, &converter_config.shex2sparql_config(), force_overwrite)
        }
        (InputConvertMode::ShEx, OutputConvertMode::UML) => {
            run_shex2uml(input_path, format, output, result_format, &converter_config.shex2uml_config(), force_overwrite)
        }
        (InputConvertMode::ShEx, OutputConvertMode::HTML) => {
            match target_folder {
                None => Err(anyhow!(
            "Conversion from ShEx to HTML requires an output parameter to indicate where to write the generated HTML files"
                )),
                Some(output_path) => {
                    run_shex2html(input_path, format, output_path, &converter_config.shex2html_config())
                }
            }
        }
        (InputConvertMode::DCTAP, OutputConvertMode::UML, ) => {
            run_tap2uml(input_path, format, output, result_format, &converter_config, force_overwrite)
        }
        (InputConvertMode::DCTAP, OutputConvertMode::HTML) => {
            match target_folder {
                None => Err(anyhow!(
            "Conversion from DCTAP to HTML requires an output parameter to indicate where to write the generated HTML files"
                )),
                Some(output_path) => {
                    run_tap2html(input_path, format, output_path, &converter_config)
                }
            }
        }
        _ => Err(anyhow!(
            "Conversion from {input_mode} to {output_mode} is not supported yet"
        )),
    }
}

fn run_shex2uml(
    input_path: &Path,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &ShEx2UmlConfig,
    force_overwrite: bool,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let schema = parse_schema(input_path, &schema_format)?;
    let mut converter = ShEx2Uml::new(config);
    converter.convert(&schema)?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    generate_uml_output(converter, &mut writer, result_format)?;
    Ok(())
}

fn generate_uml_output(
    uml_converter: ShEx2Uml,
    writer: &mut Box<dyn Write>,
    result_format: &OutputConvertFormat,
) -> Result<()> {
    match result_format {
        OutputConvertFormat::PlantUML => {
            uml_converter.as_plantuml(writer)?;
            Ok(())
        }
        OutputConvertFormat::SVG => {
            uml_converter.as_image(writer, ImageFormat::SVG)?;
            Ok(())
        }
        OutputConvertFormat::PNG => {
            uml_converter.as_image(writer, ImageFormat::PNG)?;
            Ok(())
        }
        OutputConvertFormat::Default => {
            uml_converter.as_plantuml(writer)?;
            Ok(())
        }
        // OutputConvertFormat::JPG => converter.as_image(writer, Image::JPG)?,
        _ => Err(anyhow!(
            "Conversion to UML does not support output format {result_format}"
        )),
    }
}

fn run_shex2html<P: AsRef<Path>>(
    input_path: P,
    format: &InputConvertFormat,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    config: &ShEx2HtmlConfig,
) -> Result<()> {
    debug!("Starting shex2html");
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let schema = parse_schema(input_path.as_ref(), &schema_format)?;
    let config = config.clone().with_target_folder(output_folder.as_ref());
    let landing_page = config.landing_page().to_string_lossy().to_string();
    debug!("Landing page {landing_page}\nConverter...");
    let mut converter = ShEx2Html::new(config);
    converter.convert(&schema)?;
    converter.export_schema()?;
    debug!("HTML pages generated at {}", landing_page);
    Ok(())
}

fn run_tap2html<P: AsRef<Path>>(
    input_path: P,
    format: &InputConvertFormat,
    // msg_writer: &mut Box<dyn Write>,
    output_folder: P,
    config: &ConverterConfig,
) -> Result<()> {
    debug!("Starting tap2html");
    let dctap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input_path, &dctap_format, &config.tap_config())?;
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

fn run_shex2sparql(
    input_path: &Path,
    format: &InputConvertFormat,
    shape: Option<IriRef>,
    output: &Option<PathBuf>,
    _result_format: &OutputConvertFormat,
    config: &ShEx2SparqlConfig,
    force_overwrite: bool,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let schema = parse_schema(input_path, &schema_format)?;
    let converter = ShEx2Sparql::new(config);
    let sparql = converter.convert(&schema, shape)?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    write!(writer, "{}", sparql)?;
    Ok(())
}

fn run_tap2shex(
    input_path: &Path,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &ConverterConfig,
    force_overwrite: bool,
) -> Result<()> {
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input_path, &tap_format, &config.tap_config())?;
    let converter = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter.convert(&dctap)?;
    let result_schema_format = match result_format {
        OutputConvertFormat::Default => Ok(ShExFormat::ShExC),
        OutputConvertFormat::Internal => Ok(ShExFormat::Internal),
        OutputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
        OutputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
        _ => Err(anyhow!("Can't write ShEx in {result_format} format")),
    }?;
    let (writer, color) = get_writer(output, force_overwrite)?;
    show_schema(&shex, &result_schema_format, writer, color)?;
    Ok(())
}

fn run_tap2uml(
    input_path: &Path,
    format: &InputConvertFormat,
    output: &Option<PathBuf>,
    result_format: &OutputConvertFormat,
    config: &ConverterConfig,
    force_overwrite: bool,
) -> Result<()> {
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input_path, &tap_format, &config.tap_config())?;
    let converter_shex = Tap2ShEx::new(&config.tap2shex_config());
    let shex = converter_shex.convert(&dctap)?;
    let mut converter_uml = ShEx2Uml::new(&config.shex2uml_config());
    converter_uml.convert(&shex)?;
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    converter_uml.as_plantuml(&mut writer)?;
    generate_uml_output(converter_uml, &mut writer, result_format)?;
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
                    OpenOptions::new().write(true).open(path)
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
    _debug: u8,
) -> Result<Data> {
    match (data.is_empty(), endpoint) {
        (true, None) => {
            bail!("None of `data` or `endpoint` parameters have been specified for validation")
        }
        (false, None) => {
            // let data_path = cast_to_data_path(data)?;
            let data = parse_data(data, data_format)?;
            Ok(Data::RDFData(data))
        }
        (true, Some(endpoint)) => {
            let endpoint = SRDFSparql::from_str(endpoint)?;
            Ok(Data::Endpoint(endpoint))
        }
        (false, Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' supported at the same time at this moment")
        }
    }
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
    node_str: &str,
    predicates: &Vec<String>,
    show_node_mode: &ShowNodeMode,
    show_hyperlinks: &bool,
    debug: u8,
    output: &Option<PathBuf>,
    _config: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let data = get_data(data, data_format, endpoint, debug)?;
    let node_selector = parse_node_selector(node_str)?;
    match data {
        Data::Endpoint(endpoint) => show_node_info(
            node_selector,
            predicates,
            &endpoint,
            show_node_mode,
            show_hyperlinks,
            &mut writer,
        ),
        Data::RDFData(data) => show_node_info(
            node_selector,
            predicates,
            &data,
            show_node_mode,
            show_hyperlinks,
            &mut writer,
        ),
    }
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
                    match rdf.outgoing_arcs_from_list(&subject, preds) {
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
    shapemap_path: &Path,
    shapemap_format: &ShapeMapFormat,
    result_format: &ShapeMapFormat,
    output: &Option<PathBuf>,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let shapemap = parse_shapemap(shapemap_path, shapemap_format)?;
    match result_format {
        ShapeMapFormat::Compact => {
            let str = ShapemapFormatter::default().format_shapemap(&shapemap);
            writeln!(writer, "{str}")?;
            Ok(())
        }
        ShapeMapFormat::Internal => {
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

fn run_data(
    data: &Vec<InputSpec>,
    data_format: &DataFormat,
    debug: u8,
    output: &Option<PathBuf>,
    result_format: &DataFormat,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let data = get_data(data, data_format, &None, debug)?;
    match data {
        Data::Endpoint(e) => writeln!(writer, "Endpoint {e:?}")?,
        Data::RDFData(graph) => graph.serialize(RDFFormat::from(*result_format), writer)?,
    }
    Ok(())
}

fn parse_shapemap(shapemap_path: &Path, shapemap_format: &ShapeMapFormat) -> Result<QueryShapeMap> {
    match shapemap_format {
        ShapeMapFormat::Internal => Err(anyhow!("Cannot read internal ShapeMap format yet")),
        ShapeMapFormat::Compact => {
            let shapemap = ShapeMapParser::parse_buf(shapemap_path, &None, &None)?;
            Ok(shapemap)
        }
    }
}

fn parse_schema(schema_path: &Path, schema_format: &ShExFormat) -> Result<SchemaJson> {
    match schema_format {
        ShExFormat::Internal => Err(anyhow!("Cannot read internal ShEx format yet")),
        ShExFormat::ShExC => {
            let schema = ShExParser::parse_buf(schema_path, None)?;
            Ok(schema)
        }
        ShExFormat::ShExJ => {
            let schema_json = SchemaJson::parse_schema_buf(schema_path)?;
            //let mut schema: CompiledSchema = CompiledSchema::new();
            // schema.from_schema_json(&schema_json)?;
            // Ok((&schema_json, &schema))
            Ok(schema_json)
        }
        ShExFormat::Turtle => {
            let rdf = parse_data(&vec![InputSpec::path(schema_path)], &DataFormat::Turtle)?;
            let schema = ShExRParser::new(rdf).parse()?;
            Ok(schema)
        }
        _ => Err(anyhow!("Not suppported parsing from {schema_format} yet")),
    }
}

fn parse_shacl(shapes_path: &Path, shapes_format: &ShaclFormat) -> Result<ShaclSchema> {
    match shapes_format {
        ShaclFormat::Internal => Err(anyhow!("Cannot read internal ShEx format yet")),
        _ => {
            let data_format = shacl_format_to_data_format(shapes_format)?;
            let rdf = parse_data(&vec![InputSpec::path(shapes_path)], &data_format)?;
            let schema = ShaclParser::new(rdf).parse()?;
            Ok(schema)
        }
    }
}

fn parse_dctap<P: AsRef<Path>>(
    input_path: P,
    format: &DCTapFormat,
    config: &TapConfig,
) -> Result<DCTap> {
    match format {
        DCTapFormat::CSV => {
            let dctap = DCTap::from_path(input_path, config)?;
            debug!("DCTAP read {dctap:?}");
            Ok(dctap)
        }
    }
}

fn shacl_format_to_data_format(shacl_format: &ShaclFormat) -> Result<DataFormat> {
    match shacl_format {
        ShaclFormat::Turtle => Ok(DataFormat::Turtle),
        ShaclFormat::RDFXML => Ok(DataFormat::RDFXML),
        ShaclFormat::NTriples => Ok(DataFormat::NTriples),
        ShaclFormat::TriG => Ok(DataFormat::TriG),
        ShaclFormat::N3 => Ok(DataFormat::N3),
        ShaclFormat::NQuads => Ok(DataFormat::NQuads),
        ShaclFormat::Internal => bail!("Cannot convert internal SHACL format to RDF data format"),
    }
}

fn parse_data(data: &Vec<InputSpec>, data_format: &DataFormat) -> Result<SRDFGraph> {
    let mut graph = SRDFGraph::new();
    for d in data {
        match d {
            InputSpec::Path(data_path) => match data_format {
                DataFormat::Turtle => {
                    let rdf_format = (*data_format).into();
                    // let graph = SRDFGraph::from_path(data, &rdf_format, None)?;
                    graph.merge_from_path(data_path, &rdf_format, None)?;
                    // Ok(graph)
                }
                _ => bail!("Not implemented reading from other RDF formats yet..."),
            },
            InputSpec::Stdin => bail!("Not implemented input from Stdin yet"),
            InputSpec::Url(url) => bail!("Not implemented input from URLs yet. {url:?}"),
        }
    }
    Ok(graph)
}

fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let ns = ShapeMapParser::parse_node_selector(node_str)?;
    Ok(ns)
}

fn parse_shape_label(label_str: &str) -> Result<ShapeSelector> {
    let selector = ShapeMapParser::parse_shape_selector(label_str)?;
    Ok(selector)
}

fn parse_iri_ref(iri: &str) -> Result<IriRef> {
    let iri = ShapeMapParser::parse_iri_ref(iri)?;
    Ok(iri)
}

fn cast_to_data_path(data: &Vec<InputSpec>) -> Result<Option<PathBuf>> {
    match &data[..] {
        [elem] => match elem {
            InputSpec::Path(path) => Ok(Some(path.clone())),
            InputSpec::Stdin => bail!("Not supported data from stdin yet"),
            InputSpec::Url(url) => bail!("Not supported data from url yet. Url: {url}"),
        },
        [] => Ok(None),
        _ => bail!("More than one value for data: {data:?}"),
    }
}
