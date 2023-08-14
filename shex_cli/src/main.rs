extern crate clap;
extern crate shex_ast;
extern crate anyhow;

use log::debug;
use std::{path::PathBuf, io::BufReader, fs::File};
use clap::Parser;
use anyhow::Result;
use srdf_oxgraph::{SRDFGraph, SRDFGraphError};
use rio_api::parser::*;
use rio_turtle::*;

pub mod cli;
pub use cli::*;

use shex_ast::{SchemaJson, CompiledSchema, ShapeLabel, CompiledSchemaError};

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    
    match &cli.command {
        Some(Command::Schema{ schema, schema_format}) => run_schema(schema, schema_format, cli.debug),
        Some(Command::Validate{ schema, schema_format, data, data_format, node, shape}) => run_validate(schema, schema_format, data, data_format, node, shape, cli.debug),
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
    // let node = parse_node(node_str);
    // let shape = parse_shape_label(shape_str);
    println!("Validation performed:\nData\n{data:?}\nSchema\n{schema:?}");
    Ok(())
}


fn parse_schema(schema: &PathBuf, schema_format: &ShExFormat, debug: u8) -> Result<CompiledSchema, CompiledSchemaError> {
    match schema_format {
      ShExFormat::ShExC => todo!(),
      ShExFormat::ShExJ => {
        let schema_json = SchemaJson::parse_schema_buf(schema, debug)?;
        let mut schema: CompiledSchema = CompiledSchema::new();
        debug!("Parsing schema...");
        schema.from_schema_json(schema_json)?;
        Ok(schema)
      }
    }
}

fn parse_data(data: &PathBuf, data_format: &DataFormat, debug: u8) -> Result<SRDFGraph, SRDFGraphError> {
    match data_format {
      DataFormat::Turtle => {
        let graph = SRDFGraph::parse_turtle(data, None)?;
        Ok(graph)
      }
    }
}