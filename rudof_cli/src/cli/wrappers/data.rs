use crate::cli_wrapper;
use rudof_lib_refactored::formats::{DataReaderMode, DataFormat, ResultDataFormat};
use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

// CLI wrapper for rudof_lib::rdf_reader_mode::RDFReaderMode.
cli_wrapper!(
    DataReaderModeCli,
    DataReaderMode,
    {
        Lax,
        Strict
    }
);

// CLI wrapper for rudof_lib::data_format::DataFormat.
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

// CLI wrapper for rudof_lib::result_data_format::ResultDataFormat.
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