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
    Shapemap {
        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: PathBuf,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            long = "result-shapemap-format",
            value_name = "Result shapemap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        result_shapemap_format: ShapeMapFormat,
    },

    Schema {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(
            short = 'r',
            long = "result-schema-format",
            value_name = "Result schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        result_schema_format: ShExFormat,
    },

    Validate {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap file name")]
        shapemap: Option<PathBuf>,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            long = "result-shapemap-format",
            value_name = "Result shapemap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        result_shapemap_format: ShapeMapFormat,

        #[arg(short = 'n', long = "node")]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        data: Option<PathBuf>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
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
        data: Option<PathBuf>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            short = 'm',
            long = "show-node-mode",
            value_name = "Show Node Mode",
            default_value_t = ShowNodeMode::Outgoing
        )]
        show_node_mode: ShowNodeMode,

        #[arg(long = "show hyperlinks")]
        show_hyperlinks: bool,

        #[arg(short = 'p', long = "predicates")]
        predicates: Vec<String>,
    },

    Shacl {
        #[arg(short = 's', long = "shapes", value_name = "Shapes file name")]
        shapes: PathBuf,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "Shapes file format",
            default_value_t = ShaclFormat::Turtle
        )]
        shapes_format: ShaclFormat,

        #[arg(
            short = 'r',
            long = "result-shapes-format",
            value_name = "Result shapes format",
            default_value_t = ShaclFormat::Internal
        )]
        result_shapes_format: ShaclFormat,

    },

}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShowNodeMode {
    Outgoing,
    Incoming,
    Both,
}

impl Display for ShowNodeMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShowNodeMode::Outgoing => write!(dest, "outgoing"),
            ShowNodeMode::Incoming => write!(dest, "incoming"),
            ShowNodeMode::Both => write!(dest, "both"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    Internal,
    ShExC,
    ShExJ,
    Turtle,
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::Internal => write!(dest, "internal"),
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
            ShExFormat::Turtle => write!(dest, "turtle"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShapeMapFormat {
    Compact,
    Internal,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
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


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShaclFormat {
    Internal,
    Turtle,
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
        }
    }
}