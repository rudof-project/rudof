use crate::cli_wrapper;
use rudof_lib_refactored::formats::{ComparisonFormat, ComparisonMode, ResultComparisonFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    ComparisonModeCli,
    ComparisonMode,
    {
        Shacl,
        ShEx,
        Dctap,
        Service,
    }
);

cli_wrapper!(
    ComparisonFormatCli,
    ComparisonFormat,
    {
        ShExC,
        ShExJ,
        Turtle,
        RdfXml,
        NTriples,
    }
);

cli_wrapper!(
    ResultComparisonFormatCli,
    ResultComparisonFormat,
    {
        Internal,
        Json,
    }
);