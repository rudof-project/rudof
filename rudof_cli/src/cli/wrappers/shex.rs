use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::ShExFormat;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    ShExFormatCli,
    ShExFormat,
    {
        Internal,
        Simple,
        ShExC,
        ShExJ,
        Json,
        JsonLd,
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads
    }
);
