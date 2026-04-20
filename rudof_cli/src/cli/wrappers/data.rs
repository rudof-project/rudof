use crate::cli_wrapper;
use clap::ValueEnum;
use rudof_lib::formats::{DataFormat, DataReaderMode, ResultDataFormat};
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
        JsonLd,
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
