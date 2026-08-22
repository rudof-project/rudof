use crate::cli::parser::CommonArgsNoBackend;
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `sparql` command
#[derive(Debug, Clone, Args)]
pub struct SparqlArgs {
    #[arg(
        short = 'q',
        long = "query",
        value_name = "INPUT",
        help = "SPARQL query, FILE, URI or - for stdin. If omitted, shows the currently loaded query"
    )]
    pub query: Option<InputSpec>,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
