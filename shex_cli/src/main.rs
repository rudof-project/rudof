extern crate clap;
extern crate shex_ast;
extern crate anyhow;

use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
pub mod cli;

pub use cli::*;
use shex_ast::{SchemaJson, CompiledSchema, ShapeLabel, CompiledSchemaError};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Schema{ schema, schema_format}) =>    run_schema(schema, schema_format, cli.debug),
        None => {
            println!("Command not specified");
            Ok(())
        }
    }
}

fn run_schema(schema: &PathBuf, schema_format: &ShExFormat, debug: u8) -> Result<()> {
    let schema = parse_schema(schema, schema_format, debug)?;
    println!("Schema {schema:?}");
    Ok(())
}

fn parse_schema(schema: &PathBuf, schema_format: &ShExFormat, debug: u8) -> Result<CompiledSchema<ShapeLabel>, CompiledSchemaError> {
    match schema_format {
      ShExFormat::ShExC => todo!(),
      ShExFormat::ShExJ => {
        let schema_json = SchemaJson::parse_schema_buf(schema, debug)?;
        let schema: CompiledSchema<ShapeLabel> = CompiledSchema::from_schema_json(schema_json)?;
        Ok(schema)
      }
    }
}