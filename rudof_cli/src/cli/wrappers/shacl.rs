use crate::cli_wrapper;
use rudof_lib_refactored::formats::ShaclFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    ShaclFormatCli,
    ShaclFormat,
    {
        Internal,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        JsonLd,
    }
);