use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{DCTapFormatCli, ResultDCTapFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;

/// Arguments for the `dctap` command
#[derive(Debug, Clone, Args)]
pub struct DCTapArgs {
    #[arg(short = 's', long = "source-file", value_name = "FILE", help = "DCTap source file")]
    pub file: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "DCTap file format",
        default_value_t = DCTapFormatCli::Csv
    )]
    pub format: DCTapFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Ouput results format",
        default_value_t = ResultDCTapFormatCli::Internal
    )]
    pub result_format: ResultDCTapFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
