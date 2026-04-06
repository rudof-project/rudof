use crate::cli::parser::CommonArgsAll;
use crate::cli::wrappers::{
    ConversionFormatCli, ConversionModeCli, DataReaderModeCli, ResultConversionFormatCli, ResultConversionModeCli,
};
use clap::Args;
use rudof_lib::formats::InputSpec;
use std::path::PathBuf;

/// Arguments for the `convert` command
#[derive(Debug, Clone, Args)]
pub struct ConvertArgs {
    #[arg(
        short = 'm',
        long = "input-mode",
        ignore_case = true,
        value_name = "MODE",
        help = "Input mode"
    )]
    pub input_mode: ConversionModeCli,

    #[arg(
        short = 's',
        long = "source-file",
        value_name = "INPUT",
        help = "Source file name (URI, file or - for stdin)"
    )]
    pub file: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Input file format",
        default_value_t = ConversionFormatCli::ShExC
    )]
    pub format: ConversionFormatCli,

    #[arg(
        short = 'b',
        long = "base",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs)"
    )]
    pub base: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result format",
        default_value_t = ResultConversionFormatCli::Default
    )]
    pub result_format: ResultConversionFormatCli,

    #[arg(short = 't', long = "target-folder", value_name = "FOLDER", help = "Target folder")]
    pub target_folder: Option<PathBuf>,

    #[arg(
        short = 'e',
        long = "templates-folder",
        ignore_case = true,
        value_name = "FOLDER",
        help = "Templates folder"
    )]
    pub template_folder: Option<PathBuf>,

    #[arg(
        short = 'l',
        long = "shape-label",
        value_name = "LABEL",
        help = "shape label (default = START)"
    )]
    pub shape: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        short = 'x',
        long = "export-mode",
        ignore_case = true,
        value_name = "MODE",
        help = "Result mode for conversion"
    )]
    pub output_mode: ResultConversionModeCli,

    #[arg(long = "show-time", help = "Show processing time")]
    pub show_time: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}
