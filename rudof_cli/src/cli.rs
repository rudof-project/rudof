use crate::data_format::DataFormat;
use crate::dctap_format::DCTapFormat;
use crate::input_spec::InputSpec;
use crate::{
    DCTapResultFormat, InputConvertFormat, InputConvertMode, OutputConvertFormat,
    OutputConvertMode, RDFReaderMode, ResultDataFormat, ResultQueryFormat, ResultServiceFormat,
    ResultShExValidationFormat, ResultShaclValidationFormat, ResultValidationFormat, ShExFormat,
    ShaclFormat, ShapeMapFormat, ShowNodeMode, ValidationMode,
};
use clap::{Parser, Subcommand};
use shacl_validation::shacl_processor::ShaclValidationMode;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
// #[command(name = "rudof")]
// #[command(author = "Jose Emilio Labra Gayo <labra@uniovi.es>")]
// #[command(version = "0.1")]
#[command(
    arg_required_else_help = true,
    long_about = "\
A tool to process and validate RDF data using shapes, and convert between different RDF data models"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Show information about ShEx ShapeMaps
    Shapemap {
        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap")]
        shapemap: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result shapemap format",
            default_value_t = ShapeMapFormat::Compact
        )]
        result_shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about ShEx schemas
    Shex {
        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "Schema format",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result schema format",
            default_value_t = ShExFormat::ShExJ
        )]
        result_schema_format: ShExFormat,

        #[arg(short = 't', long = "show-time")]
        show_time: Option<bool>,

        #[arg(long = "show-schema")]
        show_schema: Option<bool>,

        #[arg(long = "statistics")]
        show_statistics: Option<bool>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            long = "show-dependencies",
            value_name = "Show dependencies between shapes"
        )]
        show_dependencies: Option<bool>,

        #[arg(
            long = "compile",
            value_name = "Compile Schema to Internal representation"
        )]
        compile: Option<bool>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,
    },

    /// Validate RDF data using ShEx or SHACL
    Validate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(short = 'M', long = "mode", 
            value_name = "Validation mode",
            default_value_t = ValidationMode::ShEx
        )]
        validation_mode: ValidationMode,

        #[arg(short = 's', long = "schema", value_name = "Schema file name")]
        schema: Option<InputSpec>,

        #[arg(short = 'f', long = "schema-format", value_name = "Schema format")]
        schema_format: Option<ShExFormat>,

        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap")]
        shapemap: Option<InputSpec>,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(short = 'n', long = "node")]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            long = "max-steps",
            value_name = "max steps to run",
            default_value_t = 100
        )]
        max_steps: usize,

        /// Execution mode
        #[arg(
            short = 'S',
            long = "shacl-mode",
            value_name = "SHACL validation mode",
            default_value_t = ShaclValidationMode::Native,
            value_enum
        )]
        shacl_validation_mode: ShaclValidationMode,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput result format",
            default_value_t = ResultValidationFormat::Compact
        )]
        result_format: ResultValidationFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,
    },

    /// Validate RDF using ShEx schemas
    ShexValidate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 's',
            long = "schema",
            value_name = "Schema file name, URI or -"
        )]
        schema: Option<InputSpec>,

        #[arg(short = 'f', long = "schema-format", value_name = "Schema format")]
        schema_format: Option<ShExFormat>,

        #[arg(short = 'm', long = "shapemap", value_name = "ShapeMap")]
        shapemap: Option<InputSpec>,

        #[arg(
            long = "shapemap-format",
            value_name = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(short = 'n', long = "node")]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput result format",
            default_value_t = ResultShExValidationFormat::Turtle
        )]
        result_format: ResultShExValidationFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Validate RDF data using SHACL shapes
    ShaclValidate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 's',
            long = "shapes",
            value_name = "Shapes graph: file, URI or -, if not set, it assumes the shapes come from the data"
        )]
        shapes: Option<InputSpec>,

        #[arg(short = 'f', long = "shapes-format", value_name = "Shapes file format")]
        shapes_format: Option<ShaclFormat>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        /// Execution mode
        #[arg(
            short = 'm',
            long = "mode",
            value_name = "Execution mode",
            default_value_t = ShaclValidationMode::Native,
            value_enum
        )]
        mode: ShaclValidationMode,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput result format",
            default_value_t = ResultShaclValidationFormat::Compact
        )]
        result_format: ResultShaclValidationFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,
    },

    /// Show information about RDF data
    Data {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        // #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        // data: PathBuf,
        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput result format",
            default_value_t = ResultDataFormat::Turtle
        )]
        result_format: ResultDataFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about a node in an RDF Graph
    Node {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(short = 'n', long = "node")]
        node: String,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'm',
            long = "show-node-mode",
            value_name = "Show Node Mode",
            default_value_t = ShowNodeMode::Outgoing
        )]
        show_node_mode: ShowNodeMode,

        #[arg(long = "show hyperlinks")]
        show_hyperlinks: bool,

        #[arg(short = 'p', long = "predicates")]
        predicates: Vec<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(short = 'c', long = "config", value_name = "Path to config file")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about SHACL shapes
    Shacl {
        #[arg(
            short = 's',
            long = "shapes",
            value_name = "Shapes graph (file, URI or -)"
        )]
        shapes: InputSpec,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "Shapes file format",
            default_value_t = ShaclFormat::Turtle
        )]
        shapes_format: ShaclFormat,

        #[arg(
            short = 'r',
            long = "result-shapes-format",
            value_name = "Result shapes format",
            default_value_t = ShaclFormat::Internal
        )]
        result_shapes_format: ShaclFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,
    },

    /// Show information and process DCTAP files
    #[command(name = "dctap")]
    DCTap {
        #[arg(short = 's', long = "source-file", value_name = "DCTap source file")]
        file: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "DCTap file format",
            default_value_t = DCTapFormat::CSV
        )]
        format: DCTapFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Ouput results format",
            default_value_t = DCTapResultFormat::Internal
        )]
        result_format: DCTapResultFormat,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Convert between different Data modeling technologies
    #[command(name = "convert")]
    Convert {
        #[arg(short = 'c', long = "config", value_name = "Path to config file")]
        config: Option<PathBuf>,

        #[arg(short = 'm', long = "input-mode", value_name = "Input mode")]
        input_mode: InputConvertMode,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,

        #[arg(short = 's', long = "source-file", value_name = "Source file name")]
        file: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "Input file format",
            default_value_t = InputConvertFormat::ShExC
        )]
        format: InputConvertFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result format",
            default_value_t = OutputConvertFormat::Default
        )]
        result_format: OutputConvertFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(short = 't', long = "target-folder", value_name = "Target folder")]
        target_folder: Option<PathBuf>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "shape label (default = START)"
        )]
        shape: Option<String>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'x', long = "export-mode", value_name = "Result mode")]
        output_mode: OutputConvertMode,

        #[arg(long = "show-time")]
        show_time: Option<bool>,
    },

    /// Show information about SPARQL service
    Service {
        #[arg(short = 's', long = "service", value_name = "SPARQL service name")]
        service: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "SPARQL service format",
            default_value_t = DataFormat::Turtle
        )]
        service_format: DataFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result service format",
            default_value_t = ResultServiceFormat::Internal
        )]
        result_service_format: ResultServiceFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Run SPARQL queries
    Query {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        // #[arg(short = 'd', long = "data", value_name = "RDF data path")]
        // data: PathBuf,
        #[arg(
            short = 't',
            long = "data-format",
            value_name = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(short = 'q', long = "query", value_name = "SPARQL query")]
        query: InputSpec,

        #[arg(short = 'e', long = "endpoint", value_name = "Endpoint with RDF data")]
        endpoint: Option<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Result query format",
            default_value_t = ResultQueryFormat::Internal
        )]
        result_query_format: ResultQueryFormat,

        /// Config file path, if unset it assumes default config
        #[arg(short = 'c', long = "config-file", value_name = "Config file name")]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },
}
