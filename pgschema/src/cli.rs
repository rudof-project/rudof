use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(name = "pgspc")]
// #[command(author = "Jose Emilio Labra Gayo <labra@uniovi.es>")]
// #[command(version = "0.1")]
#[command(
    arg_required_else_help = true,
    long_about = "\
A simple prototype tool to process and validate PG-Schemas with property constraints"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(name = "pgs", about = "Process and validate property graph schemas")]
    Pgs {
        #[arg(short, long, help = "Path to the schema file")]
        schema: String,
    },
    #[command(name = "pg", about = "Process and validate property graphs")]
    Pg {
        #[arg(short, long, help = "Path to the property graph file")]
        graph: String,
    },
    #[command(name = "map", about = "Process and validate type map associations")]
    TypeMap {
        #[arg(short, long, help = "Path to the type map associations file")]
        map: String,
    },
    #[command(
        name = "validate",
        about = "Validate a property graph with a property graph schema and some associated type map"
    )]
    Validate {
        #[arg(short, long, help = "Path to the property graph file")]
        graph: String,
        #[arg(short, long, help = "Path to the property graph schema file")]
        schema: String,
        #[arg(short, long, help = "Path to the type map associations file")]
        map: String,
    },
}
