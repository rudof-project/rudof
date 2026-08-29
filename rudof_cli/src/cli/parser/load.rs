use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ShaclFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;
use std::path::PathBuf;

/// Arguments for the `load` command
///
/// Copies RDF data into a LadybugDB property graph database. By default the
/// data is validated against SHACL shapes before loading; the database DDL
/// (node/rel tables) enforces conformance for subsequent mutations.
#[derive(Debug, Clone, Args)]
pub struct LoadArgs {
    /// RDF data files to load
    #[clap(value_parser = clap::value_parser!(InputSpec), required = true)]
    pub data: Vec<InputSpec>,

    /// Path to the LadybugDB database directory (overrides connection details)
    #[arg(long, value_name = "PATH", conflicts_with = "connection")]
    pub db: Option<PathBuf>,

    /// Connection details file written by `rudof connect` (default: .rudof-connection.toml)
    #[arg(long, value_name = "FILE")]
    pub connection: Option<PathBuf>,

    /// Skip SHACL validation and just copy the data (the database DDL enforces conformance)
    #[arg(long)]
    pub skip_validation: bool,

    /// RDF data format
    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    /// Base IRI for data
    #[arg(long = "base-data", value_name = "IRI")]
    pub base_data: Option<String>,

    /// RDF Reader mode
    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    /// SHACL shapes file
    #[arg(
        short = 's',
        long = "shapes",
        value_name = "INPUT",
        help = "Shapes graph: file, URI or -, if not set, it assumes the shapes come from the data"
    )]
    pub shapes: Option<InputSpec>,

    /// SHACL shapes format
    #[arg(
        short = 'f',
        long = "shapes-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Shapes file format",
        default_value_t = ShaclFormatCli::Turtle
    )]
    pub shapes_format: ShaclFormatCli,

    /// Base IRI for shapes
    #[arg(long = "base-shapes", value_name = "IRI")]
    pub base_shapes: Option<String>,
}
