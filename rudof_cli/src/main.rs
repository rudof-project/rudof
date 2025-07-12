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
    OutputConvertMode, RDFReaderMode, ResultFormat, ResultServiceFormat, ValidationMode,
};
use dctap::DCTAPFormat;
use iri_s::IriS;
use prefixmap::IriRef;
use rudof_lib::{
    Rudof, RudofConfig, ShExFormat, ShExFormatter, ShaclFormat, ShaclValidationMode,
    ShapeMapFormatter, ShapeMapParser, ShapesGraphSource,
};
use shacl_validation::validation_report::report::ValidationReport;
use shapemap::{ResultShapeMap, ShapeMapFormat as ShapemapFormat, ShapeSelector};
use shapes_converter::ShEx2Sparql;
use shapes_converter::{ImageFormat, ShEx2Html, ShEx2Uml, Shacl2ShEx, Tap2ShEx, UmlGenerationMode};
use sparql_service::ServiceDescription;
use srdf::{RDFFormat, ReaderMode, SRDFGraph};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::result::Result::Ok;
use tracing::debug;

// Current modules
pub mod cli;
pub mod data;
pub mod input_convert_format;
pub mod input_spec;
pub mod node;
pub mod node_selector;
pub mod output_convert_format;
pub mod query;
pub mod shex;
pub mod writer;

pub use cli::{
    ShExFormat as CliShExFormat, ShaclFormat as CliShaclFormat, ShapeMapFormat as CliShapeMapFormat,
};
pub use input_convert_format::InputConvertFormat;
pub use input_spec::*;
pub use output_convert_format::OutputConvertFormat;

use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, fmt};

use crate::data::{data_format2rdf_format, get_base, get_data_rudof, run_data};
use crate::node::run_node;
use crate::node_selector::parse_node_selector;
use crate::query::run_query;
use crate::shex::{parse_shex_schema_rudof, run_shex, show_shex_schema_rudof};
use crate::writer::get_writer;

#[allow(unused_variables)]
fn main() -> Result<()> {
    // Load environment variables from `.env`:
    clientele::dotenv().ok();

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

    // Expand wildcards and @argfiles:
    let args = clientele::args_os()?;

    // Parse command-line options:
    let cli = Cli::parse_from(args);

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
            show_dependencies,
            output,
            show_time,
            show_schema,
            show_statistics,
            compile,
            force_overwrite,
            reader_mode,
            config,
        }) => {
            let config = get_config(config)?;
            if let Some(show_dependencies) = show_dependencies {
                config
                    .shex_config()
                    .with_show_dependencies(*show_dependencies);
            }
            if let Some(flag) = show_statistics {
                config.shex_config().set_show_extends(*flag);
            }
            let show_time = (*show_time).unwrap_or_default();
            run_shex(
                schema,
                schema_format,
                result_schema_format,
                output,
                show_time,
                show_schema.unwrap_or_default(),
                compile.unwrap_or_default(),
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
            result_format,
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
                    result_format,
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
                        result_format,
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
            result_format,
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
                result_format,
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
            result_format,
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
                result_format,
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
        }) => {
            let config = get_config(config)?;
            run_dctap(
                file,
                format,
                result_format,
                output,
                &config,
                *force_overwrite,
            )?;
            Ok(())
        }
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
            show_time,
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
            show_time.unwrap_or(false),
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
            let config = get_config(config)?;
            run_query(
                data,
                data_format,
                endpoint,
                reader_mode,
                query,
                result_query_format,
                output,
                &config,
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
    let reader = input.open_read(Some(data_format.mime_type().as_str()), "Service")?;
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
    result_format: &ResultFormat,
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
        let base_iri = config.shex_config().base;
        let schema_base = base_iri.as_ref().map(|iri| iri.as_str());
        rudof.read_shex(schema_reader, &schema_format, schema_base)?;
        get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;

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
        write_result_shapemap(writer, result_format, result)?;
        Ok(())
    } else {
        bail!("No ShEx schema specified")
    }
}

fn write_validation_report(
    mut writer: Box<dyn Write + 'static>,
    format: &ResultFormat,
    report: ValidationReport,
) -> Result<()> {
    match format {
        ResultFormat::Compact => {
            writeln!(writer, "Validation report: {report}")?;
        }
        ResultFormat::Json => {
            bail!("Generation of JSON for SHACl validation report is not implemented yet")
            /*let str = serde_json::to_string_pretty(&report)
                .context("Error converting Result to JSON: {result}")?;
            writeln!(writer, "{str}")?;*/
        }
        _ => {
            use crate::srdf::BuildRDF;
            let mut rdf_writer = SRDFGraph::new();
            report.to_rdf(&mut rdf_writer)?;
            let rdf_format = result_format_to_rdf_format(format)?;
            rdf_writer.serialize(&rdf_format, &mut writer)?;
        }
    }
    Ok(())
}

fn result_format_to_rdf_format(result_format: &ResultFormat) -> Result<RDFFormat> {
    match result_format {
        ResultFormat::Turtle => Ok(RDFFormat::Turtle),
        ResultFormat::NTriples => Ok(RDFFormat::NTriples),
        ResultFormat::RDFXML => Ok(RDFFormat::RDFXML),
        ResultFormat::TriG => Ok(RDFFormat::TriG),
        ResultFormat::N3 => Ok(RDFFormat::N3),
        ResultFormat::NQuads => Ok(RDFFormat::NQuads),
        _ => bail!("Unsupported result format {result_format}"),
    }
}

fn write_result_shapemap(
    mut writer: Box<dyn Write + 'static>,
    format: &ResultFormat,
    result: ResultShapeMap,
) -> Result<()> {
    match format {
        ResultFormat::Turtle => todo!(),
        ResultFormat::NTriples => todo!(),
        ResultFormat::RDFXML => todo!(),
        ResultFormat::TriG => todo!(),
        ResultFormat::N3 => todo!(),
        ResultFormat::NQuads => todo!(),
        ResultFormat::Compact => {
            writeln!(writer, "Result:")?;
            result.show_minimal(writer)?;
        }
        ResultFormat::Json => {
            let str = serde_json::to_string_pretty(&result)
                .context("Error converting Result to JSON: {result}")?;
            writeln!(writer, "{str}")?;
        }
    }
    Ok(())
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
    result_format: &ResultFormat,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    get_data_rudof(&mut rudof, data, data_format, endpoint, reader_mode, config)?;
    let validation_report = if let Some(schema) = schema {
        let reader_mode = reader_mode_convert(*reader_mode);
        let shapes_format = shapes_format.unwrap_or_default();
        add_shacl_schema_rudof(&mut rudof, schema, &shapes_format, &reader_mode, config)?;
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_schema())
    } else {
        rudof.validate_shacl(&mode, &ShapesGraphSource::current_data())
    }?;

    write_validation_report(writer, result_format, validation_report)?;

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
    let shacl_format = shacl_format_convert(result_shapes_format)?;
    rudof.serialize_shacl(&shacl_format, &mut writer)?;
    Ok(())
}

fn run_dctap(
    input: &InputSpec,
    format: &DCTapFormat,
    result_format: &DCTapResultFormat,
    output: &Option<PathBuf>,
    config: &RudofConfig,
    force_overwrite: bool,
) -> Result<()> {
    let (mut writer, _color) = get_writer(output, force_overwrite)?;
    let mut rudof = Rudof::new(config);
    parse_dctap(&mut rudof, input, format)?;
    if let Some(dctap) = rudof.get_dctap() {
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
    } else {
        bail!("Internal error: No DCTAP read")
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
    show_time: bool,
) -> Result<()> {
    // let mut writer = get_writer(output)?;
    let mut config = get_config(config)?;
    match (input_mode, output_mode) {
        (InputConvertMode::ShEx, OutputConvertMode::ShEx) => {
            let shex_format = format_2_shex_format(format)?;
            let output_format = output_format_2_shex_format(result_format)?;
            config.shex_without_showing_stats();
            run_shex(input, &shex_format, &output_format, output, show_time, true, false, force_overwrite, reader_mode, &config)
        }
        (InputConvertMode::SHACL, OutputConvertMode::SHACL) => {
            let shacl_format = format_2_shacl_format(format)?;
            let output_format = output_format_2_shacl_format(result_format)?;
            run_shacl(input, &shacl_format, &output_format, output, force_overwrite, reader_mode, &config)
        }
        (InputConvertMode::DCTAP, OutputConvertMode::ShEx) => {
            run_tap2shex(input, format, output, result_format, &config, force_overwrite)
        }
        (InputConvertMode::ShEx, OutputConvertMode::SPARQL) => {
            let maybe_shape = match maybe_shape_str {
                None => None,
                Some(shape_str) => {
                    let iri_shape = ShapeMapParser::parse_iri_ref(shape_str)?;
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
    show_shex_schema_rudof(
        &rudof,
        // converter.current_shex(),
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
    if let Some(schema) = rudof.get_shex() {
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
    if let Some(schema) = rudof.get_shex() {
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
    let mut rudof = Rudof::new(config);
    let dctap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(DCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    parse_dctap(&mut rudof, input, &dctap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter_tap = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter_tap.convert(dctap)?;
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
    } else {
        bail!("Internal error: no DCTAP")
    }
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
    if let Some(schema) = rudof.get_shex() {
        let converter = ShEx2Sparql::new(&config.shex2sparql_config());
        let sparql = converter.convert(schema, shape)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        write!(writer, "{sparql}")?;
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
    let mut rudof = Rudof::new(config);
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(DCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    parse_dctap(&mut rudof, input_path, &tap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter.convert(dctap)?;
        let result_schema_format = match result_format {
            OutputConvertFormat::Default => Ok(CliShExFormat::ShExC),
            OutputConvertFormat::Internal => Ok(CliShExFormat::Internal),
            OutputConvertFormat::ShExJ => Ok(CliShExFormat::ShExJ),
            OutputConvertFormat::Turtle => Ok(CliShExFormat::Turtle),
            _ => Err(anyhow!("Can't write ShEx in {result_format} format")),
        }?;
        let (writer, color) = get_writer(output, force_overwrite)?;
        show_shex_schema_rudof(&rudof, &result_schema_format, writer, color)?;
        Ok(())
    } else {
        bail!("Internal error: No DCTAP")
    }
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
    let mut rudof = Rudof::new(config);
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        InputConvertFormat::Xlsx => Ok(DCTapFormat::XLSX),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    parse_dctap(&mut rudof, input_path, &tap_format)?;
    if let Some(dctap) = rudof.get_dctap() {
        let converter_shex = Tap2ShEx::new(&config.tap2shex_config());
        let shex = converter_shex.convert(dctap)?;
        let mut converter_uml = ShEx2Uml::new(&config.shex2uml_config());
        converter_uml.convert(&shex)?;
        let (mut writer, _color) = get_writer(output, force_overwrite)?;
        generate_uml_output(converter_uml, maybe_shape, &mut writer, result_format)?;
        Ok(())
    } else {
        bail!("Internal error: No DCTAP")
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ColorSupport {
    NoColor,
    WithColor,
}

fn add_shacl_schema_rudof(
    rudof: &mut Rudof,
    schema: &InputSpec,
    shapes_format: &CliShaclFormat,
    reader_mode: &ReaderMode,
    config: &RudofConfig,
) -> Result<()> {
    let reader = schema.open_read(Some(shapes_format.mime_type().as_str()), "SHACL shapes")?;
    let shapes_format = shacl_format_convert(shapes_format)?;
    let base = get_base(schema, config)?;
    rudof.read_shacl(reader, &shapes_format, base.as_deref(), reader_mode)?;
    Ok(())
}

fn start() -> ShapeSelector {
    ShapeSelector::start()
}

fn run_shapemap(
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

fn parse_dctap(rudof: &mut Rudof, input: &InputSpec, format: &DCTapFormat) -> Result<()> {
    let dctap_format = match format {
        DCTapFormat::CSV => DCTAPFormat::CSV,
        DCTapFormat::XLSX => DCTAPFormat::XLSX,
        DCTapFormat::XLSB => DCTAPFormat::XLSB,
        DCTapFormat::XLSM => DCTAPFormat::XLSM,
        DCTapFormat::XLS => DCTAPFormat::XLS,
    };
    match format {
        DCTapFormat::CSV => {
            let reader = input.open_read(None, "DCTAP")?;
            rudof.read_dctap(reader, &dctap_format)?;
            Ok(())
        }
        _ => match input {
            InputSpec::Path(path_buf) => {
                rudof.read_dctap_path(path_buf, &dctap_format)?;
                Ok(())
            }
            InputSpec::Stdin => bail!("Can not read Excel file from stdin"),
            InputSpec::Url(_) => bail!("Not implemented reading Excel files from URIs yet"),
            InputSpec::Str(_) => {
                bail!("Not implemented reading Excel files from strings yet")
            }
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
        cli::ShaclFormat::Internal => Ok(ShaclFormat::Internal),
    }
}

fn parse_shape_selector(label_str: &str) -> Result<ShapeSelector> {
    let selector = ShapeMapParser::parse_shape_selector(label_str)?;
    Ok(selector)
}

fn get_config(config: &Option<PathBuf>) -> Result<RudofConfig> {
    match config {
        Some(config_path) => match RudofConfig::from_path(config_path) {
            Ok(c) => Ok(c),
            Err(e) => Err(anyhow!(
                "Error obtaining Rudof config from {}\nError: {e}",
                config_path.display()
            )),
        },
        None => Ok(RudofConfig::default()),
    }
}

/*fn get_query_config(config: &Option<PathBuf>) -> Result<QueryConfig> {
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
}*/

fn shapemap_format_convert(shapemap_format: &CliShapeMapFormat) -> ShapemapFormat {
    match shapemap_format {
        CliShapeMapFormat::Compact => ShapemapFormat::Compact,
        CliShapeMapFormat::Internal => ShapemapFormat::JSON,
    }
}

fn output_format_2_shacl_format(format: &OutputConvertFormat) -> Result<CliShaclFormat> {
    match format {
        OutputConvertFormat::Default => Ok(CliShaclFormat::Internal),
        OutputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
        _ => bail!("Converting SHACL, format {format} not supported"),
    }
}

fn output_format_2_shex_format(format: &OutputConvertFormat) -> Result<CliShExFormat> {
    match format {
        OutputConvertFormat::Default => Ok(CliShExFormat::ShExC),
        OutputConvertFormat::ShExC => Ok(CliShExFormat::ShExC),
        OutputConvertFormat::ShExJ => Ok(CliShExFormat::ShExJ),
        OutputConvertFormat::Turtle => Ok(CliShExFormat::Turtle),
        _ => bail!("Converting ShEx, format {format} not supported"),
    }
}

fn format_2_shex_format(format: &InputConvertFormat) -> Result<CliShExFormat> {
    match format {
        InputConvertFormat::ShExC => Ok(CliShExFormat::ShExC),
        InputConvertFormat::ShExJ => Ok(CliShExFormat::ShExJ),
        InputConvertFormat::Turtle => Ok(CliShExFormat::Turtle),
        _ => bail!("Converting ShEx, format {format} not supported"),
    }
}

fn format_2_shacl_format(format: &InputConvertFormat) -> Result<CliShaclFormat> {
    match format {
        InputConvertFormat::Turtle => Ok(CliShaclFormat::Turtle),
        _ => bail!("Converting ShEx, format {format} not supported"),
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
