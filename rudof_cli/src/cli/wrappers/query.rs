use crate::cli_wrapper;
use rudof_lib_refactored::formats::{QueryType, ResultQueryFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::query_type::QueryType.
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

// CLI wrapper for rudof_lib::query_result_format::ResultQueryFormat.
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