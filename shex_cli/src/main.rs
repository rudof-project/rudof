extern crate anyhow;
extern crate clap;
extern crate iri_s;
extern crate log;
extern crate oxrdf;
extern crate regex;
extern crate serde_json;
extern crate shapemap;
extern crate shex_ast;
extern crate shex_compact;
extern crate shex_validation;
extern crate srdf;
extern crate srdf_graph;
extern crate srdf_sparql;
extern crate prefixmap;

use anyhow::*;
use clap::Parser;
use iri_s::*;
use log::debug;
use oxrdf::{BlankNode, NamedNode, Subject};
use shapemap::{query_shape_map::QueryShapeMap, NodeSelector, ShapeSelector};
use shex_ast::{Node, object_value::ObjectValue, ShapeExprLabel};
use shex_compact::{ShExFormatter, ShExParser, ShapeMapParser, ShapemapFormatter};
use shex_validation::Validator;
use srdf::{Object, SRDF};
use srdf_graph::SRDFGraph;
use srdf_sparql::SRDFSparql;
use std::{path::PathBuf, str::FromStr};

pub mod cli;
pub mod data;

pub use cli::*;
pub use data::*;

use shex_ast::{ast::Schema as SchemaJson, internal::CompiledSchema};

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Schema {
            schema,
            schema_format,
            result_schema_format,
        }) => run_schema(schema, schema_format, result_schema_format),
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
            result_shapemap_format
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
        ),
        Some(Command::Data { data, data_format }) => run_data(data, data_format, cli.debug),
        Some(Command::Node {
            data,
            data_format,
            endpoint,
            node,
        }) => run_node(data, data_format, endpoint, node, cli.debug),
        Some(Command::Shapemap {
            shapemap,
            shapemap_format,
            result_shapemap_format,
        }) => run_shapemap(shapemap, shapemap_format, result_shapemap_format),
        None => {
            println!("Command not specified");
            Ok(())
        }
    }
}

fn run_schema(
    schema_buf: &PathBuf,
    schema_format: &ShExFormat,
    result_schema_format: &ShExFormat,
) -> Result<()> {
    let schema_json = parse_schema(schema_buf, schema_format)?;
    match result_schema_format {
        ShExFormat::Internal => {
            println!("{schema_json:?}");
            Ok(())
        }
        ShExFormat::ShExC => {
            let str = ShExFormatter::default().format_schema(&schema_json);
            println!("{str}");
            Ok(())
        }
        ShExFormat::ShExJ => {
            let str = serde_json::to_string_pretty(&schema_json)?;
            println!("{str}");
            Ok(())
        }
    }
}

fn run_validate(
    schema_path: &PathBuf,
    schema_format: &ShExFormat,
    data: &Option<PathBuf>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    maybe_node: &Option<String>,
    maybe_shape: &Option<String>,
    shapemap: &Option<PathBuf>,
    shapemap_format: &ShapeMapFormat,
    max_steps: &usize,
    debug: u8,
) -> Result<()> {
    let schema_json = parse_schema(schema_path, schema_format)?;
    let mut schema: CompiledSchema = CompiledSchema::new();
    schema.from_schema_json(&schema_json)?;
    let data = get_data(data, data_format, endpoint, debug)?;
    let mut shapemap = match shapemap {
        None => {
            QueryShapeMap::new()
        },
        Some(shapemap_buf) => {
            parse_shapemap(shapemap_buf, shapemap_format)?
        }
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
            debug!("Shape label {shape_str} ignored because noshapemap has also been provided")
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
                println!("Result:\n{}", result_map);
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

fn get_data(data: &Option<PathBuf>,
    data_format: &DataFormat,
    endpoint: &Option<String>,
    debug: u8
) -> Result<Data> {
    match (data, endpoint) {
            (None, None) => bail!("None of `data` or `endpoint` parameters have been specified for validation"),
            (Some(data), None) => {
             let data = parse_data(data, data_format, debug)?;
             Ok(Data::RDFData(data))
            },
            (None, Some(endpoint)) => {
             Ok(Data::Endpoint(SRDFSparql::new(endpoint)))
            },
            (Some(_), Some(_)) => bail!("Only one of 'data' or 'endpoint' parameters supported at the same time")
         }
}


fn make_node_selector(node: Node) -> Result<NodeSelector> {
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

fn start() -> ShapeSelector {
    ShapeSelector::start()
}


fn run_node(data: &Option<PathBuf>, 
    data_format: &DataFormat, 
    endpoint: &Option<String>, 
    node_str: &String, 
    debug: u8) -> Result<()> {
    let data = get_data(data, data_format, endpoint, debug)?;
    let node_selector = parse_node_selector(node_str)?;
    match data {
        Data::Endpoint(endpoint) => show_node_info(node_selector, endpoint),
        Data::RDFData(data) => show_node_info(node_selector, data),
    }
}

fn show_node_info<S>(node_selector: NodeSelector, rdf: &S) -> Result<()> 
where S: SRDF  {
    for node in node_selector.iter_node(rdf) {
        let subject = node_to_subject(node)?;
        let preds = rdf.get_predicates_for_subject(&subject)?;
        println!("Information about node");
        println!("{}", rdf.qualify_subject(&subject));
        for pred in preds {
            println!("  {} {}", rdf.qualify_named_node(&pred), &pred);
            let objs = rdf.get_objects_for_subject_predicate(&subject, &pred)?;
            for o in objs {
                println!("     {}", rdf.qualify_term(&o));
            }
        }
    }
    Ok(())
}

fn run_shapemap(
    shapemap: &PathBuf,
    shapemap_format: &ShapeMapFormat,
    result_format: &ShapeMapFormat,
) -> Result<()> {
    let shapemap = parse_shapemap(shapemap, shapemap_format)?;
    match result_format {
        ShapeMapFormat::Compact => {
            let str = ShapemapFormatter::default().format_shapemap(&shapemap);
            println!("{str}");
            Ok(())
        }
        ShapeMapFormat::Internal => {
            println!("{shapemap:?}");
            Ok(())
        }
    }
}

fn node_to_subject(node: &Object) -> Result<Subject> {
    match node {
        Object::BlankNode(bn) => Ok(Subject::BlankNode(BlankNode::new_unchecked(bn.as_str()))),
        Object::Iri { iri } => Ok(Subject::NamedNode(NamedNode::new_unchecked(iri.as_str()))),
        Object::Literal(_lit) => Err(anyhow!("Node must be an IRI or a blank node")),
    }
}

fn run_data(data: &PathBuf, data_format: &DataFormat, debug: u8) -> Result<()> {
    let data = parse_data(data, data_format, debug)?;
    println!("Data\n{data:?}\n");
    Ok(())
}

fn parse_shapemap(
    shapemap_path: &PathBuf,
    shapemap_format: &ShapeMapFormat,
) -> Result<QueryShapeMap> {
    match shapemap_format {
        ShapeMapFormat::Internal => Err(anyhow!("Cannot read internal ShapeMap format yet")),
        ShapeMapFormat::Compact => {
            let shapemap = ShapeMapParser::parse_buf(shapemap_path, &None, &None)?;
            Ok(shapemap)
        }
    }
}

fn parse_schema(schema_path: &PathBuf, schema_format: &ShExFormat) -> Result<SchemaJson> {
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
    }
}

fn parse_data(data: &PathBuf, data_format: &DataFormat, _debug: u8) -> Result<SRDFGraph> {
    match data_format {
        DataFormat::Turtle => {
            let graph = SRDFGraph::from_path(data, None)?;
            Ok(graph)
        }
    }
}

fn parse(node_str: &str, data: &SRDFGraph) -> Result<Node> {
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
}

fn parse_node_selector(node_str: &str) -> Result<NodeSelector> {
    let ns = ShapeMapParser::parse_node_selector(node_str)?;
    Ok(ns)
}


fn parse_shape_label(label_str: &str) -> Result<ShapeSelector> {
    let selector = ShapeMapParser::parse_shape_selector(label_str)?;
    Ok(selector)
}
