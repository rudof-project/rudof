use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::{QueryType, ResultQueryFormat};
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    QueryTypeCli,
    QueryType,
    {
        Select,
        Construct,
        Ask,
        Describe,
    }
);

cli_wrapper!(
    ResultQueryFormatCli,
    ResultQueryFormat,
    {
        Internal,
        Turtle,
        NTriples,
        JsonLd,
        RdfXml,
        Csv,
        TriG,
        N3,
        NQuads,
    }
);
