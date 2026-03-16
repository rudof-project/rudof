use crate::cli_wrapper;
use rudof_lib_refactored::formats::ShaclFormat;
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::shacl_format::ShaclFormat.
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