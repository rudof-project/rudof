extern crate clap;
extern crate shex_ast;
extern crate anyhow;

use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use srdf_graph::{SRDFGraph, SRDFGraphError};
use oxrdf::{Subject, NamedNode};
use iri_s::*;
use srdf::SRDF;

pub mod cli;
pub use cli::*;

use shex_ast::{SchemaJson, compiled_schema::CompiledSchema, ShapeLabel, CompiledSchemaError};

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Command::Schema{ 
          schema, 
          schema_format
        }) => run_schema(schema, schema_format, cli.debug),
        Some(Command::Validate{ 
          schema, 
          schema_format, 
          data, 
          data_format, 
          node, 
          shape
        }) => run_validate(schema, schema_format, data, data_format, node, shape, cli.debug),
        Some(Command::Data{ 
          data, 
          data_format, 
        }) => run_data(data, data_format, cli.debug),
        Some(Command::Node{ 
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
    debug: u8) -> Result<()> {
    let schema = parse_schema(schema, schema_format, debug)?;
    let data = parse_data(data, data_format, debug)?;
    let node = parse_node(node_str, &data)?;
    let shape = parse_shape_label(shape_str)?;
    println!("Validation performed:\nData\n{data:?}\nSchema\n{schema:?}\nNode: {node}\nShape: {shape}");
    Ok(())
}

fn run_node(
  data: &PathBuf, 
  data_format: &DataFormat, 
  node_str: &String, 
  debug: u8) -> Result<()> {
  let data = parse_data(data, data_format, debug)?;
  let node = parse_node(node_str, &data)?;
  let subject = Subject::NamedNode(node.clone());
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


fn run_data(
  data: &PathBuf, 
  data_format: &DataFormat, 
  debug: u8) -> Result<()> {
  let data = parse_data(data, data_format, debug)?;
  println!("Data\n{data:?}\n");
  Ok(())
}

fn parse_schema(schema: &PathBuf, schema_format: &ShExFormat, debug: u8) -> Result<CompiledSchema, CompiledSchemaError> {
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

fn parse_data(data: &PathBuf, data_format: &DataFormat, _debug: u8) -> Result<SRDFGraph, SRDFGraphError> {
    match data_format {
      DataFormat::Turtle => {
        let graph = SRDFGraph::from_path(data, None)?;
        Ok(graph)
      }
    }
}

fn parse_node(node_str: &str, data: &SRDFGraph) -> Result<NamedNode> {
  use regex::Regex;
  let iri_r = Regex::new("<(.*)>")?;
  match iri_r.captures(node_str) {
    Some(captures) => {
     match captures.get(1) {
      Some(cs) => {
        let iri = NamedNode::new(cs.as_str())?;
        Ok(iri)
      },
    None => {
      todo!()
    } 
   }
  },
  None => {
    match data.resolve(node_str) {
      Ok(Some(iri)) => Ok(iri),
      Ok(None) => {
        todo!()
      },
      Err(_err_resolve) => {
        todo!()
      } 
    }
  }
}
}
  


fn parse_shape_label(label_str: &str) -> Result<ShapeLabel> {
  let iri = IriS::new(label_str)?;
  Ok(ShapeLabel::Iri(iri))
}