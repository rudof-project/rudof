use crate::cli_wrapper;
use rudof_lib_refactored::formats::{
    ConversionFormat, ConversionMode, ResultConversionFormat, ResultConversionMode
};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};


// CLI wrapper for rudof_lib::convert::InputConvertMode.
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

// CLI wrapper for rudof_lib::convert::InputConvertFormat.
cli_wrapper!(
    ConversionModeCli,
    ConversionMode,
    {
        Shacl,
        ShEx,
        Dctap,
    }
);

// CLI wrapper for rudof_lib::convert::OutputConvertFormat.
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

// CLI wrapper for rudof_lib::convert::OutputConvertMode.
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