use std::fmt::Display;
use std::{fmt::Formatter, path::PathBuf};

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(name = "shex-cli")]
// #[command(author = "Jose Emilio Labra Gayo <labra@uniovi.es>")]
// #[command(version = "0.1")]
#[command(
    arg_required_else_help = true,
    long_about = r#"
 This tool is a work in progress implementation of ShEx in Rust"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Schema {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        schema_format: ShExFormat,
    },

    Validate {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        schema_format: ShExFormat,

        #[arg(short = 'n', long = "node")]
        node: String,

        #[arg(short = 'l', long = "shape", value_name = "shape label")]
        shape: String,

        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: PathBuf,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            short = 'm',
            long = "max-steps",
            value_name = "max steps to run",
            default_value_t = 100
        )]
        max_steps: usize,
    },

    Data {
        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: PathBuf,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,
    },

    Node {
        #[arg(short = 'n', long = "node")]
        node: String,

        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: PathBuf,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    ShExC,
    ShExJ,
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DataFormat {
    Turtle,
}

impl Display for DataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DataFormat::Turtle => write!(dest, "turtle"),
        }
    }
}
