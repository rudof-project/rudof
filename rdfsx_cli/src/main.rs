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
extern crate shex_ast;
extern crate shex_compact;
extern crate shex_validation;
extern crate srdf;
extern crate tracing;
extern crate tracing_subscriber;

use anyhow::*;
use clap::Parser;
use dctap::{DCTap, TapConfig};
use prefixmap::IriRef;
use shacl_ast::{Schema as ShaclSchema, ShaclParser, ShaclWriter};
use shapemap::{query_shape_map::QueryShapeMap, NodeSelector, ShapeSelector};
use shapes_converter::{shex_to_sparql::ShEx2SparqlConfig, ShEx2Sparql};
use shapes_converter::{ShEx2Uml, ShEx2UmlConfig, Tap2ShEx, Tap2ShExConfig};
use shex_ast::{object_value::ObjectValue, shexr::shexr_parser::ShExRParser};
use shex_compact::{ShExFormatter, ShExParser, ShapeMapParser, ShapemapFormatter};
use shex_validation::Validator;
use srdf::srdf_graph::SRDFGraph;
use srdf::srdf_sparql::SRDFSparql;
use srdf::SRDF;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::Instant;
use tracing::debug;

pub mod cli;
pub mod data;

pub use cli::*;
pub use data::*;

use shex_ast::{ast::Schema as SchemaJson, compiled::compiled_schema::CompiledSchema};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{filter::EnvFilter, fmt};

fn main() -> Result<()> {
    let fmt_layer = fmt::layer()
        .with_file(true)
        .with_target(false)
        .with_line_number(true)
        .without_time();
    // Attempts to get the value of RUST_LOG which can be info, debug, trace, If unset, it uses "info"
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    // tracing::info!("rdfsx is running...");

    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Schema {
            schema,
            schema_format,
            result_schema_format,
            output,
            show_time,
            show_statistics,
        }) => run_schema(
            schema,
            schema_format,
            result_schema_format,
            output,
            *show_time,
            *show_statistics,
        ),
        Some(Command::Validate {
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
            output,
        }) => run_validate(
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
            cli.debug,
            output,
        ),
        Some(Command::Data {
            data,
            data_format,
            output,
        }) => run_data(data, data_format, cli.debug, output),
        Some(Command::Node {
            data,
            data_format,
            endpoint,
            node,
            predicates,
            show_node_mode,
            show_hyperlinks,
            output,
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
        ),
        Some(Command::Shapemap {
            shapemap,
            shapemap_format,
            result_shapemap_format,
            output,
        }) => run_shapemap(shapemap, shapemap_format, result_shapemap_format, output),
        Some(Command::Shacl {
            shapes,
            shapes_format,
            result_shapes_format,
            output,
        }) => run_shacl(shapes, shapes_format, result_shapes_format, output),
        Some(Command::DCTap {
            file,
            format,
            result_format,
            output,
        }) => run_dctap(file, format, result_format, output),
        Some(Command::Convert {
            file,
            format,
            input_mode,
            shape,
            result_format,
            output,
            output_mode,
        }) => run_convert(
            file,
            format,
            input_mode,
            shape,
            result_format,
            output,
            output_mode,
        ),
        None => {
            println!("Command not specified");
            Ok(())
        }
    }
}

fn run_schema(
    schema_path: &Path,
    schema_format: &ShExFormat,
    result_schema_format: &ShExFormat,
    output: &Option<PathBuf>,
    show_time: bool,
    show_statistics: bool,
) -> Result<()> {
    let begin = Instant::now();
    let writer = get_writer(output)?;
    let schema_json = parse_schema(schema_path, schema_format)?;
    show_schema(&schema_json, result_schema_format, writer)?;
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
) -> Result<()> {
    match result_schema_format {
        ShExFormat::Internal => {
            writeln!(writer, "{schema:?}")?;
        }
        ShExFormat::ShExC => {
            let str = ShExFormatter::default().format_schema(schema);
            writeln!(writer, "{str}")?;
        }
        ShExFormat::ShExJ => {
            let str = serde_json::to_string_pretty(&schema)?;
            writeln!(writer, "{str}")?;
        }
        ShExFormat::Turtle => {
            eprintln!("Not implemented conversion to Turtle yet");
            todo!()
        }
    };
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn run_validate(
    schema_path: &Path,
    schema_format: &ShExFormat,
    data: &Option<PathBuf>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap_path: &Option<PathBuf>,
    shapemap_format: &ShapeMapFormat,
    max_steps: &usize,
    debug: u8,
    output: &Option<PathBuf>,
) -> Result<()> {
    let mut writer = get_writer(output)?;
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
    let mut validator = Validator::new(schema).with_max_steps(*max_steps);
    debug!("Validating with max_steps: {}", max_steps);
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

fn run_shacl(
    shapes_path: &Path,
    shapes_format: &ShaclFormat,
    result_shapes_format: &ShaclFormat,
    output: &Option<PathBuf>,
) -> Result<()> {
    let mut writer = get_writer(output)?;
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
) -> Result<()> {
    let mut writer = get_writer(output)?;
    let dctap = parse_dctap(input_path, format)?;
    match result_format {
        DCTapResultFormat::Internal => {
            writeln!(writer, "{dctap:?}")?;
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

fn run_convert(
    input_path: &Path,
    format: &InputConvertFormat,
    input_mode: &InputConvertMode,
    maybe_shape_str: &Option<String>,
    result_format: &OutputConvertFormat,
    output: &Option<PathBuf>,
    output_mode: &OutputConvertMode,
) -> Result<()> {
    let mut writer = get_writer(output)?;
    match (input_mode, output_mode) {
        (InputConvertMode::DCTAP, OutputConvertMode::ShEx) => {
            run_tap2shex(input_path, format, writer, result_format)
        }
        (InputConvertMode::ShEx, OutputConvertMode::SPARQL) => {
            let maybe_shape = match maybe_shape_str {
                None => None,
                Some(shape_str) => {
                    let iri_shape = parse_iri_ref(shape_str)?;
                    Some(iri_shape)
                }
            };
            run_shex2sparql(input_path, format, maybe_shape, &mut writer, result_format)
        }
        (InputConvertMode::ShEx, OutputConvertMode::UML) => {
            run_shex2uml(input_path, format, &mut writer, result_format)
        }
        _ => Err(anyhow!(
            "Conversion from {input_mode} to {output_mode} is not supported yet"
        )),
    }
}

fn run_shex2uml(
    input_path: &Path,
    format: &InputConvertFormat,
    writer: &mut Box<dyn Write>,
    _result_format: &OutputConvertFormat,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let schema = parse_schema(input_path, &schema_format)?;
    let mut converter = ShEx2Uml::new(ShEx2UmlConfig::default());
    converter.convert(&schema)?;
    converter.as_plantuml(writer)?;
    Ok(())
}

fn run_shex2sparql(
    input_path: &Path,
    format: &InputConvertFormat,
    shape: Option<IriRef>,
    writer: &mut Box<dyn Write>,
    _result_format: &OutputConvertFormat,
) -> Result<()> {
    let schema_format = match format {
        InputConvertFormat::ShExC => Ok(ShExFormat::ShExC),
        _ => Err(anyhow!("Can't obtain ShEx format from {format}")),
    }?;
    let schema = parse_schema(input_path, &schema_format)?;
    let converter = ShEx2Sparql::new(ShEx2SparqlConfig::default());
    let sparql = converter.convert(&schema, shape)?;
    write!(writer, "{}", sparql)?;
    Ok(())
}

fn run_tap2shex(
    input_path: &Path,
    format: &InputConvertFormat,
    writer: Box<dyn Write>,
    result_format: &OutputConvertFormat,
) -> Result<()> {
    let tap_format = match format {
        InputConvertFormat::CSV => Ok(DCTapFormat::CSV),
        _ => Err(anyhow!("Can't obtain DCTAP format from {format}")),
    }?;
    let dctap = parse_dctap(input_path, &tap_format)?;
    let converter = Tap2ShEx::new(Tap2ShExConfig::default());
    let shex = converter.convert(&dctap)?;
    let result_schema_format = match result_format {
        OutputConvertFormat::Default => Ok(ShExFormat::ShExC),
        OutputConvertFormat::Internal => Ok(ShExFormat::Internal),
        OutputConvertFormat::ShExJ => Ok(ShExFormat::ShExJ),
        OutputConvertFormat::Turtle => Ok(ShExFormat::Turtle),
        _ => Err(anyhow!("Can't write ShEx in {result_format} format")),
    }?;
    show_schema(&shex, &result_schema_format, writer)?;
    Ok(())
}

fn get_writer(output: &Option<PathBuf>) -> Result<Box<dyn Write>> {
    match output {
        None => {
            let stdout = io::stdout();
            let handle = stdout.lock();
            Ok(Box::new(handle))
        }
        Some(path) => {
            let file = File::create(path)?;
            let writer = BufWriter::new(file);
            Ok(Box::new(writer))
        }
    }
}

fn get_data(
    data: &Option<PathBuf>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    _debug: u8,
) -> Result<Data> {
    match (data, endpoint) {
        (None, None) => {
            bail!("None of `data` or `endpoint` parameters have been specified for validation")
        }
        (Some(data), None) => {
            let data = parse_data(data, data_format)?;
            Ok(Data::RDFData(data))
        }
        (None, Some(endpoint)) => {
            let endpoint = SRDFSparql::from_str(endpoint)?;
            Ok(Data::Endpoint(endpoint))
        }
        (Some(_), Some(_)) => {
            bail!("Only one of 'data' or 'endpoint' parameters supported at the same time")
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
    data: &Option<PathBuf>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    node_str: &str,
    predicates: &Vec<String>,
    show_node_mode: &ShowNodeMode,
    show_hyperlinks: &bool,
    debug: u8,
    output: &Option<PathBuf>,
) -> Result<()> {
    let mut writer = get_writer(output)?;
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
) -> Result<()> {
    let mut writer = get_writer(output)?;
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
    data: &Path,
    data_format: &DataFormat,
    _debug: u8,
    output: &Option<PathBuf>,
) -> Result<()> {
    let mut writer = get_writer(output)?;
    let data = parse_data(data, data_format)?;
    writeln!(writer, "Data\n{data:?}\n")?;
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
            let rdf = parse_data(schema_path, &DataFormat::Turtle)?;
            let schema = ShExRParser::new(rdf).parse()?;
            Ok(schema)
        }
    }
}

fn parse_shacl(shapes_path: &Path, shapes_format: &ShaclFormat) -> Result<ShaclSchema> {
    match shapes_format {
        ShaclFormat::Internal => Err(anyhow!("Cannot read internal ShEx format yet")),
        _ => {
            let data_format = shacl_format_to_data_format(shapes_format)?;
            let rdf = parse_data(shapes_path, &data_format)?;
            let schema = ShaclParser::new(rdf).parse()?;
            Ok(schema)
        }
    }
}

fn parse_dctap(input_path: &Path, format: &DCTapFormat) -> Result<DCTap> {
    match format {
        DCTapFormat::CSV => {
            let dctap = DCTap::from_path(input_path, TapConfig::default())?;
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

fn parse_data(data: &Path, data_format: &DataFormat) -> Result<SRDFGraph> {
    match data_format {
        DataFormat::Turtle => {
            let rdf_format = (*data_format).into();
            let graph = SRDFGraph::from_path(data, &rdf_format, None)?;
            Ok(graph)
        }
        _ => bail!("Not implemented reading from other RDF formats yet..."),
    }
}

/*fn parse(node_str: &str, data: &SRDFGraph) -> Result<Node> {
    use regex::Regex;
    use std::result::Result::Ok;
    let iri_r = Regex::new("<(.*)>")?;
    match iri_r.captures(node_str) {
        Some(captures) => match captures.get(1) {
            Some(cs) => {
                let iri = IriS::from_str(cs.as_str())?;
                Ok(iri.into())
            }
            None => {
                todo!()
            }
        },
        None => match data.resolve(node_str) {
            Ok(named_node) => {
                let iri = IriS::from_str(named_node.as_str())?;
                Ok(iri.into())
            }
            Err(_err_resolve) => {
                todo!()
            }
        },
    }
}*/

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
