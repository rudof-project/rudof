extern crate anyhow;
extern crate clap;
extern crate shex_ast;

use anyhow::*;
use clap::Parser;
use iri_s::*;
use oxrdf::{BlankNode, NamedNode, Subject, Term};
use shex_validation::Validator;
use srdf::{Object, SRDF};
use srdf_graph::{SRDFGraph, SRDFGraphError};
use std::path::PathBuf;

pub mod cli;
pub use cli::*;

use shex_ast::{compiled_schema::CompiledSchema, CompiledSchemaError, SchemaJson, ShapeLabel};

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Schema {
            schema,
            schema_format,
        }) => run_schema(schema, schema_format, cli.debug),
        Some(Command::Validate {
            schema,
            schema_format,
            data,
            data_format,
            node,
            shape,
        }) => run_validate(
            schema,
            schema_format,
            data,
            data_format,
            node,
            shape,
            cli.debug,
        ),
        Some(Command::Data { data, data_format }) => run_data(data, data_format, cli.debug),
        Some(Command::Node {
            data,
            data_format,
            node,
        }) => run_node(data, data_format, node, cli.debug),
        None => {
            println!("Command not specified");
            Ok(())
        }
    }
}

fn run_schema(schema: &PathBuf, schema_format: &ShExFormat, debug: u8) -> Result<()> {
    let schema = parse_schema(schema, schema_format, debug)?;
    println!("Compiled Schema\n{schema}");
    Ok(())
}

fn run_validate(
    schema: &PathBuf,
    schema_format: &ShExFormat,
    data: &PathBuf,
    data_format: &DataFormat,
    node_str: &String,
    shape_str: &String,
    debug: u8,
) -> Result<()> {
    let schema = parse_schema(schema, schema_format, debug)?;
    let data = parse_data(data, data_format, debug)?;
    let node = parse_node(node_str, &data)?;
    let shape = parse_shape_label(shape_str)?;
    let mut validator = Validator::new(schema).with_max_steps(1);
    match validator.validate_node_shape(node, shape, &data) {
        Result::Ok(t) => {
            println!("Result: {:?}", validator.result_map());
            Ok(())
        }
        Result::Err(err) => {
            bail!("Error: {err}");
        }
    }
}

fn run_node(data: &PathBuf, data_format: &DataFormat, node_str: &String, debug: u8) -> Result<()> {
    let data = parse_data(data, data_format, debug)?;
    let node = parse_node(node_str, &data)?;
    let subject = node_to_subject(node)?;
    let preds = data.get_predicates_for_subject(&subject)?;
    println!("Information about node");
    println!("{}", data.qualify_subject(&subject));
    for pred in preds {
        println!("  {}", data.qualify_named_node(&pred));
        let objs = data.get_objects_for_subject_predicate(&subject, &pred)?;
        for o in objs {
            println!("     {}", data.qualify_term(&o));
        }
    }
    Ok(())
}

/*fn term_to_subject(term: Term) -> Result<Subject> {
  match term {
    Term::BlankNode(bn) => Ok(Subject::BlankNode(bn)),
    Term::NamedNode(n) => Ok(Subject::NamedNode(n)),
    Term::Literal(lit) => {
      Err(anyhow!("Node must be an IRI or a blank node"))
    }
  }
}*/

fn node_to_subject(node: Object) -> Result<Subject> {
    match node {
        Object::BlankNode(bn) => Ok(Subject::BlankNode(BlankNode::new_unchecked(bn.as_str()))),
        Object::Iri { iri } => Ok(Subject::NamedNode(NamedNode::new_unchecked(iri.as_str()))),
        Object::Literal(lit) => Err(anyhow!("Node must be an IRI or a blank node")),
    }
}

fn run_data(data: &PathBuf, data_format: &DataFormat, debug: u8) -> Result<()> {
    let data = parse_data(data, data_format, debug)?;
    println!("Data\n{data:?}\n");
    Ok(())
}

fn parse_schema(schema: &PathBuf, schema_format: &ShExFormat, debug: u8) -> Result<CompiledSchema> {
    match schema_format {
        ShExFormat::ShExC => todo!(),
        ShExFormat::ShExJ => {
            let schema_json = SchemaJson::parse_schema_buf(schema, debug)?;
            let mut schema: CompiledSchema = CompiledSchema::new();
            schema.from_schema_json(schema_json)?;
            Ok(schema)
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

fn parse_node(node_str: &str, data: &SRDFGraph) -> Result<Object> {
    use regex::Regex;
    use std::result::Result::Ok;
    let iri_r = Regex::new("<(.*)>")?;
    match iri_r.captures(node_str) {
        Some(captures) => match captures.get(1) {
            Some(cs) => {
                let iri = IriS::new(cs.as_str())?;
                Ok(Object::Iri { iri })
            }
            None => {
                todo!()
            }
        },
        None => match data.resolve(node_str) {
            Ok(Some(named_node)) => {
                let iri = IriS::new(named_node.as_str())?;
                Ok(Object::Iri { iri })
            }
            Ok(None) => {
                todo!()
            }
            Err(_err_resolve) => {
                todo!()
            }
        },
    }
}

fn parse_shape_label(label_str: &str) -> Result<ShapeLabel> {
    let iri = IriS::new(label_str)?;
    Ok(ShapeLabel::Iri(iri))
}
