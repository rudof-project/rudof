use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{RdfConfigFormatCli, ResultRdfConfigFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `rdf-config` command
#[derive(Debug, Clone, Args)]
pub struct RdfConfigArgs {
    #[arg(
        short = 's',
        long = "source-file",
        value_name = "INPUT",
        help = "Source file name (URI, file or - for stdin)"
    )]
    pub input: InputSpec,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Output result rdf-config format",
        default_value_t = ResultRdfConfigFormatCli::Internal
    )]
    pub result_format: ResultRdfConfigFormatCli,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "rdf-config format",
        default_value_t = RdfConfigFormatCli::Yaml
    )]
    pub format: RdfConfigFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
