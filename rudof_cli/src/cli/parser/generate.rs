use crate::cli::parser::CommonArgsNoBackend;
use crate::cli::wrappers::{DataFormatCli, GenerationSchemaFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;
use std::path::PathBuf;

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

    /// Number of entities to generate. Defaults to 10, unless `--generator-config`
    /// is given and its `entity_count` is left to take effect instead.
    #[arg(short = 'n', long = "entities", value_name = "Number of entities to generate")]
    pub entity_count: Option<usize>,

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

    /// Generator config file (TOML or JSON) controlling entity distribution,
    /// cardinality strategy, per-field generators, etc.
    ///
    /// Distinct from `--config-file`/`-c`, which is the common rudof config
    /// (`[rdf]`, `[shex]`, ...) shared by every subcommand. `--entities`,
    /// `--result-format`, `--seed`, `--parallel` and `--output-file` all
    /// override the matching setting in this file when both are given.
    #[arg(long = "generator-config", value_name = "Generator config file (TOML or JSON)")]
    pub generator_config: Option<PathBuf>,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
