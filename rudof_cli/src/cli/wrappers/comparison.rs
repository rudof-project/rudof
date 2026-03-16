use crate::cli_wrapper;
use rudof_lib_refactored::formats::{ComparisonFormat, ComparisonMode, ResultComparisonFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::compare::InputCompareMode.
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

// CLI wrapper for rudof_lib::compare::InputCompareFormat.
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

// CLI wrapper for rudof_lib::compare::ResultCompareFormat.
cli_wrapper!(
    ResultComparisonFormatCli,
    ResultComparisonFormat,
    {
        Internal,
        Json,
    }
);