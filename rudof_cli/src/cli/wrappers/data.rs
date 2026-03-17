use crate::cli_wrapper;
use rudof_lib_refactored::formats::{DataReaderMode, DataFormat, ResultDataFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

cli_wrapper!(
    DataReaderModeCli,
    DataReaderMode,
    {
        Lax,
        Strict
    }
);

cli_wrapper!(
    DataFormatCli,
    DataFormat,
    {
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        JsonLd,
        Pg
    }
);

cli_wrapper!(
    ResultDataFormatCli,
    ResultDataFormat,
    {
        Turtle,
        NTriples,
        RdfXml,
        TriG,
        N3,
        NQuads,
        Compact,
        Json,
        PlantUML,
        Svg,
        Png,
    }
);