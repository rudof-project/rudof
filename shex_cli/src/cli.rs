use std::{path::PathBuf, fmt::Formatter};
use std::fmt::Display;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(name = "shex-cli")]
// #[command(author = "Jose Emilio Labra Gayo <labra@uniovi.es>")]
// #[command(version = "0.1")]
#[command(long_about = r#"
 This tool is a work in progress implementation of ShEx in Rust"#)]
pub struct Cli {

  #[command(subcommand)]
  pub command: Option<Command>,   

  #[arg(short, long, action = clap::ArgAction::Count)]
  pub debug: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    
    Schema {
        #[arg(
            short = 's',
            long = "schema",
            value_name = "Schema file name",
        )]
        schema: PathBuf,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        schema_format: ShExFormat,
        
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
            ShExFormat::ShExJ => write!(dest, "shexj")
        }
    }
}

