use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::ShaclFormat;
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
