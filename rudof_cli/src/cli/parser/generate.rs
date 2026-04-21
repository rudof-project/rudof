use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{DataFormatCli, GenerationSchemaFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `generate` command
#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    #[arg(short = 's', long = "schema", value_name = "Schema file (ShEx or SHACL)")]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "Schema format",
        default_value_t = GenerationSchemaFormatCli::Auto
    )]
    pub schema_format: GenerationSchemaFormatCli,

    #[arg(
        short = 'n',
        long = "entities",
        value_name = "Number of entities to generate",
        default_value_t = 10
    )]
    pub entity_count: usize,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "Output RDF format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub result_format: DataFormatCli,

    #[arg(long = "seed", value_name = "Random seed for reproducible generation")]
    pub seed: Option<u64>,

    #[arg(short = 'p', long = "parallel", value_name = "Number of parallel threads")]
    pub parallel: Option<usize>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
