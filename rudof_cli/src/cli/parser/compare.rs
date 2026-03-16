use crate::cli::wrappers::{ComparisonFormatCli, ComparisonModeCli, DataReaderModeCli, ResultComparisonFormatCli};
use crate::cli::parser::CommonArgsAll;
use clap::Args;
use rudof_lib_refactored::formats::InputSpec;
use std::path::PathBuf;

/// Arguments for the `compare` command
#[derive(Debug, Clone, Args)]
pub struct CompareArgs {
    #[arg(long = "mode1",
        value_name = "MODE",
        ignore_case = true,
        help = "Input mode first schema",
        default_value_t = ComparisonModeCli::ShEx
    )]
    pub input_mode1: ComparisonModeCli,

    #[arg(
        long = "mode2",
        value_name = "MODE",
        ignore_case = true,
        help = "Input mode second schema",
        default_value_t = ComparisonModeCli::ShEx
    )]
    pub input_mode2: ComparisonModeCli,

    #[arg(long = "schema1", value_name = "INPUT", help = "Schema 1 (URI, file or - for stdin)")]
    pub schema1: InputSpec,

    #[arg(long = "schema2", value_name = "INPUT", help = "Schema 2 (URI, file or - for stdin)")]
    pub schema2: InputSpec,

    #[arg(
        long = "format1",
        value_name = "FORMAT",
        ignore_case = true,
        help = "File format 1",
        default_value_t = ComparisonFormatCli::ShExC
    )]
    pub format1: ComparisonFormatCli,

    #[arg(
        long = "format2",
        value_name = "FORMAT",
        ignore_case = true,
        help = "File format 2",
        default_value_t = ComparisonFormatCli::ShExC
    )]
    pub format2: ComparisonFormatCli,

    #[arg(long = "base1", value_name = "IRI", help = "Base IRI for 1st Schema")]
    pub base1: Option<String>,

    #[arg(long = "base2", value_name = "IRI", help = "Base IRI for 2nd Schema")]
    pub base2: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Result format",
        default_value_t = ResultComparisonFormatCli::Internal
    )]
    pub result_format: ResultComparisonFormatCli,

    #[arg(short = 't', long = "target-folder", value_name = "FOLDER", help = "Target folder")]
    pub target_folder: Option<PathBuf>,

    #[arg(long = "shape1", value_name = "LABEL", help = "shape1 (default = START)")]
    pub shape1: Option<String>,

    #[arg(long = "shape2", value_name = "LABEL", help = "shape2 (default = START)")]
    pub shape2: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(long = "show-time", help = "Show processing time")]
    pub show_time: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}