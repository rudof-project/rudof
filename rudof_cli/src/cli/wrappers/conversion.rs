use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::{ConversionFormat, ConversionMode, ResultConversionFormat, ResultConversionMode};
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    ConversionFormatCli,
    ConversionFormat,
    {
        Csv,
        ShExC,
        ShExJ,
        Turtle,
        Xlsx,
    }
);

cli_wrapper!(
    ConversionModeCli,
    ConversionMode,
    {
        Shacl,
        ShEx,
        Dctap,
    }
);

cli_wrapper!(
    ResultConversionFormatCli,
    ResultConversionFormat,
    {
        Default,
        Internal,
        Json,
        ShExC,
        ShExJ,
        Turtle,
        PlantUML,
        Html,
        Svg,
        Png,
    }
);

cli_wrapper!(
    ResultConversionModeCli,
    ResultConversionMode,
    {
        Sparql,
        ShEx,
        Uml,
        Html,
        Shacl,
    }
);
