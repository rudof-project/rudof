use crate::cli_wrapper;
use rudof_lib_refactored::formats::{ShExFormat};
use clap::ValueEnum;
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