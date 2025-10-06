use crate::data_format::DataFormat;
use crate::dctap_format::DCTapFormat;
use crate::result_compare_format::ResultCompareFormat;
use crate::{
    CliShaclFormat, DCTapResultFormat, InputCompareFormat, InputCompareMode, InputConvertFormat,
    InputConvertMode, OutputConvertFormat, OutputConvertMode, QueryType, RDFReaderMode,
    RdfConfigFormat, RdfConfigResultFormat, ResultDataFormat, ResultQueryFormat,
    ResultServiceFormat, ResultShExValidationFormat, ResultShaclValidationFormat,
    ResultValidationFormat, ShExFormat, ShapeMapFormat, ShowNodeMode, ValidationMode,
};
use clap::{Parser, Subcommand};
use iri_s::IriS;
use rudof_lib::InputSpec;
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
        #[arg(
            short = 'm',
            long = "shapemap",
            value_name = "INPUT",
            help = "ShapeMap (FILE, URI or - for stdin"
        )]
        shapemap: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "FORMAT",
            help = "ShapeMap format, default = compact",
            default_value_t = ShapeMapFormat::Compact
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT",
            help = "Result shapemap format, default = compact",
            default_value_t = ShapeMapFormat::Compact
        )]
        result_shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "BOOL",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about ShEx schemas
    Shex {
        #[arg(
            short = 's',
            long = "schema",
            value_name = "INPUT",
            help = "Schema, FILE, URI or - for stdin"
        )]
        schema: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "FORMAT",
            help = "Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC",
            default_value_t = ShExFormat::ShExC
        )]
        schema_format: ShExFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT",
            help = "Result schema format, default = ShExJ",
            default_value_t = ShExFormat::ShExJ
        )]
        result_schema_format: ShExFormat,

        #[arg(
            short = 't',
            value_name = "BOOL",
            help = "SHow processing time",
            long = "show-time"
        )]
        show_time: Option<bool>,

        #[arg(long = "show-schema", value_name = "BOOL", help = "Show schema")]
        show_schema: Option<bool>,

        #[arg(
            long = "statistics",
            value_name = "BOOL",
            help = "Show statistics about the schema"
        )]
        show_statistics: Option<bool>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
        base: Option<IriS>,

        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode (strict or lax)",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            long = "show-dependencies",
            value_name = "BOOL",
            help = "Show dependencies between shapes"
        )]
        show_dependencies: Option<bool>,

        #[arg(
            long = "compile",
            value_name = "BOOL",
            help = "Compile Schema to Internal representation"
        )]
        compile: Option<bool>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,
    },

    /// Validate RDF data using ShEx or SHACL
    Validate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(short = 'M', long = "mode", 
            value_name = "MODE",
            help = "Validation mode (ShEx or SHACL)",
            default_value_t = ValidationMode::ShEx
        )]
        validation_mode: ValidationMode,

        #[arg(
            short = 's',
            long = "schema",
            value_name = "INPUT",
            help = "Schema used for validatio, FILE, URI or - for stdin"
        )]
        schema: Option<InputSpec>,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "FORMAT",
            help = "Schema format"
        )]
        schema_format: Option<ShExFormat>,

        #[arg(
            short = 'm',
            long = "shapemap",
            value_name = "INPUT",
            help = "ShapeMap used for validation, FILE, URI or - for stdin"
        )]
        shapemap: Option<InputSpec>,

        #[arg(
            long = "shapemap-format",
            value_name = "FORMAT", 
            help = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(long = "base-data", value_name = "IRI", help = "Base IRI for data")]
        base_data: Option<IriS>,

        #[arg(long = "base-schema", value_name = "IRI", help = "Base IRI for Schema")]
        base_schema: Option<IriS>,

        #[arg(
            short = 'n',
            long = "node",
            value_name = "NODE",
            help = "Node to validate"
        )]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "LABEL",
            help = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "FORMAT", 
            help = "RDF Data format (default = turtle)",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            short = 'e',
            long = "endpoint",
            value_name = "ENDPOINT",
            help = "Endpoint with RDF data"
        )]
        endpoint: Option<String>,

        #[arg(
            long = "max-steps",
            value_name = "NUMBER",
            help = "max steps to run during validation",
            default_value_t = 100
        )]
        max_steps: usize,

        /// Execution mode
        #[arg(
            short = 'S',
            long = "shacl-mode",
            value_name = "MODE",
            help = "SHACL validation mode (default = native)",
            default_value_t = ShaclValidationMode::Native,
            value_enum
        )]
        shacl_validation_mode: ShaclValidationMode,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", help = "Ouput result format, default = compact",
            default_value_t = ResultValidationFormat::Compact
        )]
        result_format: ResultValidationFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            default_value_t = false,
            help = "Force overwrite to output file if it already exists"
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name (in TOML format)"
        )]
        config: Option<PathBuf>,
    },

    /// Validate RDF using ShEx schemas
    ShexValidate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 's',
            long = "schema",
            value_name = "INPUT",
            help = "Schema file name, URI or - (for stdin)"
        )]
        schema: Option<InputSpec>,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "FORMAT",
            help = "ShEx Schema format"
        )]
        schema_format: Option<ShExFormat>,

        #[arg(
            short = 'm',
            long = "shapemap",
            value_name = "INPUT",
            help = "ShapeMap"
        )]
        shapemap: Option<InputSpec>,

        #[arg(
            long = "shapemap-format",
            value_name = "FORMAT", 
            help = "ShapeMap format",
            default_value_t = ShapeMapFormat::Compact,
        )]
        shapemap_format: ShapeMapFormat,

        #[arg(
            short = 'n',
            long = "node",
            value_name = "NODE",
            help = "Node to validate"
        )]
        node: Option<String>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "LABEL",
            help = "shape label (default = START)",
            group = "node_shape"
        )]
        shape: Option<String>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "FORMAT", 
            help = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            long = "base-schema",
            value_name = "IRI",
            help = "Base Schema (used to resolve relative IRIs in Schema)"
        )]
        base_schema: Option<IriS>,

        #[arg(
            long = "base-data",
            value_name = "IRI",
            help = "Base RDF Data IRI (used to resolve relative IRIs in RDF data)"
        )]
        base_data: Option<IriS>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'e',
            long = "endpoint",
            value_name = "NAME",
            help = "Endpoint with RDF data (name or URL)"
        )]
        endpoint: Option<String>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Ouput result format",
            default_value_t = ResultShExValidationFormat::Compact
        )]
        result_format: ResultShExValidationFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Validate RDF data using SHACL shapes
    ShaclValidate {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "FORMAT", 
            help= "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            long = "base-data",
            value_name = "IRI",
            help = "Base IRI (used to resolve relative IRIs in RDF data)"
        )]
        base_data: Option<IriS>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 's',
            long = "shapes",
            value_name = "INPUT",
            help = "Shapes graph: file, URI or -, if not set, it assumes the shapes come from the data"
        )]
        shapes: Option<InputSpec>,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "FORMAT",
            help = "Shapes file format"
        )]
        shapes_format: Option<CliShaclFormat>,

        #[arg(
            long = "base-shapes",
            value_name = "IRI",
            help = "Base IRI (used to resolve relative IRIs in Shapes)"
        )]
        base_shapes: Option<IriS>,

        #[arg(
            short = 'e',
            long = "endpoint",
            value_name = "ENDPOINT",
            help = "Endpoint with RDF data (URL or name)"
        )]
        endpoint: Option<String>,

        /// Execution mode
        #[arg(
            short = 'm',
            long = "mode",
            value_name = "MODE", 
            help = "Execution mode",
            default_value_t = ShaclValidationMode::Native,
            value_enum
        )]
        mode: ShaclValidationMode,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Ouput result format",
            default_value_t = ResultShaclValidationFormat::Compact
        )]
        result_format: ResultShaclValidationFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,
    },

    /// Show information about RDF data
    Data {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "FORMAT", 
            help = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
        base: Option<IriS>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Ouput result format",
            default_value_t = ResultDataFormat::Turtle
        )]
        result_format: ResultDataFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about a node in an RDF Graph
    Node {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 'n',
            long = "node",
            value_name = "Node",
            help = "Node to show information (can be a URI or prefixed name)"
        )]
        node: String,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "FORMAT",
            help = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            short = 'e',
            long = "endpoint",
            value_name = "Endpoint",
            help = "Endpoint with RDF data (URL or name)"
        )]
        endpoint: Option<String>,

        #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
        base: Option<IriS>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'm',
            long = "show-node-mode",
            value_name = "MODE", 
            help = "Mode used to show the node information",
            default_value_t = ShowNodeMode::Outgoing
        )]
        show_node_mode: ShowNodeMode,

        #[arg(long = "show hyperlinks", help = "Show hyperlinks in the output")]
        show_hyperlinks: bool,

        #[arg(
            short = 'p',
            long = "predicates",
            value_name = "PREDICATES",
            help = "List of predicates to show"
        )]
        predicates: Vec<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'c',
            long = "config",
            value_name = "FILE",
            help = "Path to config file"
        )]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Show information about SHACL shapes
    /// The SHACL schema can be passed through the data options or the optional schema options to provide an interface similar to Shacl-validate
    Shacl {
        #[clap(value_parser = clap::value_parser!(InputSpec))]
        data: Vec<InputSpec>,

        #[arg(
            short = 't',
            long = "data-format",
            value_name = "FORMAT", 
            help = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'e',
            long = "endpoint",
            value_name = "Endpoint",
            help = "Endpoint with RDF data (URL or name)"
        )]
        endpoint: Option<String>,

        #[arg(
            short = 's',
            long = "shapes",
            value_name = "INPUT",
            help = "Shapes graph: File, URI or - for stdin, if not set, it assumes the shapes come from the data"
        )]
        shapes: Option<InputSpec>,

        #[arg(
            short = 'f',
            long = "shapes-format",
            value_name = "FORMAT",
            help = "Shapes file format"
        )]
        shapes_format: Option<CliShaclFormat>,

        #[arg(
            long = "base-data",
            value_name = "IRI",
            help = "Base RDF Data (used to resolve relative IRIs in RDF data)"
        )]
        base_data: Option<IriS>,

        #[arg(
            long = "base-shapes",
            value_name = "IRI",
            help = "Base RDF Data (used to resolve relative IRIs in Shapes)"
        )]
        base_shapes: Option<IriS>,

        #[arg(
            short = 'r',
            long = "result-shapes-format",
            value_name = "FORMAT", 
            help = "Result shapes format",
            default_value_t = CliShaclFormat::Internal
        )]
        result_shapes_format: CliShaclFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,
    },

    /// Show information and process DCTAP files
    #[command(name = "dctap")]
    DCTap {
        #[arg(
            short = 's',
            long = "source-file",
            value_name = "FILE",
            help = "DCTap source file"
        )]
        file: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "FORMAT", 
            help = "DCTap file format",
            default_value_t = DCTapFormat::CSV
        )]
        format: DCTapFormat,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Ouput results format",
            default_value_t = DCTapResultFormat::Internal
        )]
        result_format: DCTapResultFormat,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Convert between different Data modeling technologies
    #[command(name = "convert")]
    Convert {
        #[arg(
            short = 'c',
            long = "config",
            value_name = "FILE",
            help = "Path to config file"
        )]
        config: Option<PathBuf>,

        #[arg(
            short = 'm',
            long = "input-mode",
            value_name = "MODE",
            help = "Input mode"
        )]
        input_mode: InputConvertMode,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,

        #[arg(
            short = 's',
            long = "source-file",
            value_name = "INPUT",
            help = "Source file name (URI, file or - for stdin)"
        )]
        file: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "FORMAT", 
            help = "Input file format",
            default_value_t = InputConvertFormat::ShExC
        )]
        format: InputConvertFormat,

        #[arg(
            short = 'b',
            long = "base",
            value_name = "IRI",
            help = "Base IRI (used to resolve relative IRIs)"
        )]
        base: Option<IriS>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Result format",
            default_value_t = OutputConvertFormat::Default
        )]
        result_format: OutputConvertFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 't',
            long = "target-folder",
            value_name = "FOLDER",
            help = "Target folder"
        )]
        target_folder: Option<PathBuf>,

        #[arg(
            short = 'l',
            long = "shape-label",
            value_name = "LABEL",
            help = "shape label (default = START)"
        )]
        shape: Option<String>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'x',
            long = "export-mode",
            value_name = "MODE",
            help = "Result mode for conversion"
        )]
        output_mode: OutputConvertMode,

        #[arg(long = "show-time", help = "Show processing time")]
        show_time: Option<bool>,
    },

    /// Compare two shapes (which can be in different formats)
    #[command(name = "compare")]
    Compare {
        #[arg(
            short = 'c',
            long = "config",
            value_name = "FILE",
            help = "Path to config file"
        )]
        config: Option<PathBuf>,

        #[arg(long = "mode1", 
         value_name = "MODE", 
         help = "Input mode first schema", 
         default_value_t = InputCompareMode::default())]
        input_mode1: InputCompareMode,

        #[arg(
            long = "mode2",
            value_name = "MODE",
            help = "Input mode second schema",
            default_value_t = InputCompareMode::default()
        )]
        input_mode2: InputCompareMode,

        #[arg(
            long = "force-overwrite",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,

        #[arg(
            long = "schema1",
            value_name = "INPUT",
            help = "Schema 1 (URI, file or - for stdin)"
        )]
        schema1: InputSpec,

        #[arg(
            long = "schema2",
            value_name = "INPUT",
            help = "Schema 2 (URI, file or - for stdin)"
        )]
        schema2: InputSpec,

        #[arg(
            long = "format1",
            value_name = "FORMAT", 
            help = "File format 1",
            default_value_t = InputCompareFormat::default()
        )]
        format1: InputCompareFormat,

        #[arg(
            long = "format2",
            value_name = "FORMAT", 
            help = "File format 2",
            default_value_t = InputCompareFormat::default()
        )]
        format2: InputCompareFormat,

        #[arg(long = "base1", value_name = "IRI", help = "Base IRI for 1st Schema")]
        base1: Option<IriS>,

        #[arg(long = "base2", value_name = "IRI", help = "Base IRI for 2nd Schema")]
        base2: Option<IriS>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Result format",
            default_value_t = ResultCompareFormat::default()
        )]
        result_format: ResultCompareFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 't',
            long = "target-folder",
            value_name = "FOLDER",
            help = "Target folder"
        )]
        target_folder: Option<PathBuf>,

        #[arg(
            long = "shape1",
            value_name = "LABEL",
            help = "shape1 (default = START)"
        )]
        shape1: Option<String>,

        #[arg(
            long = "shape2",
            value_name = "LABEL",
            help = "shape2 (default = START)"
        )]
        shape2: Option<String>,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(long = "show-time", help = "Show processing time")]
        show_time: Option<bool>,
    },

    /// Show information about SPARQL service
    RdfConfig {
        #[arg(
            short = 's',
            long = "source-file",
            value_name = "INPUT",
            help = "Source file name (URI, file or - for stdin)"
        )]
        input: InputSpec,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Output result rdf-config format",
            default_value_t = RdfConfigResultFormat::default()
        )]
        result_format: RdfConfigResultFormat,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "FORMAT",
            help = "rdf-config format",
            default_value_t = RdfConfigFormat::default()
        )]
        format: RdfConfigFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "BOOL",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,
    },

    /// Show information about SPARQL service
    Service {
        #[arg(
            short = 's',
            long = "service",
            value_name = "URL",
            help = "SPARQL service URL"
        )]
        service: InputSpec,

        #[arg(
            short = 'f',
            long = "format",
            value_name = "FORMAT",
            help = "SPARQL service format",
            default_value_t = DataFormat::Turtle
        )]
        service_format: DataFormat,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Output result service format",
            default_value_t = ResultServiceFormat::JSON
        )]
        result_service_format: ResultServiceFormat,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "BOOL",
            help = "Force overwrite to output file if it already exists",
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
            value_name = "FORMAT",
            help = "RDF Data format",
            default_value_t = DataFormat::Turtle
        )]
        data_format: DataFormat,

        #[arg(
            short = 'b',
            long = "base",
            value_name = "IRI",
            help = "Base IRI (used to resolve relative IRIs in RDF data)"
        )]
        base: Option<IriS>,

        #[arg(long = "query-type", 
            value_name = "TYPE", 
            help = "Query type (SELECT, ASK, CONSTRUCT, DESCRIBE)", 
            default_value_t = QueryType::Select,
            value_enum
        )]
        query_type: QueryType,

        /// RDF Reader mode
        #[arg(
            long = "reader-mode",
            value_name = "MODE", 
            help = "RDF Reader mode",
            default_value_t = RDFReaderMode::default(),
            value_enum
        )]
        reader_mode: RDFReaderMode,

        #[arg(
            short = 'q',
            long = "query",
            value_name = "INPUT",
            help = "SPARQL query"
        )]
        query: InputSpec,

        #[arg(
            short = 'e',
            long = "endpoint",
            value_name = "Endpoint",
            help = "Endpoint with RDF data (URL or name)"
        )]
        endpoint: Option<String>,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "FILE",
            help = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "FORMAT", 
            help = "Result query format",
            default_value_t = ResultQueryFormat::Internal
        )]
        result_query_format: ResultQueryFormat,

        /// Config file path, if unset it assumes default config
        #[arg(
            short = 'c',
            long = "config-file",
            value_name = "FILE",
            help = "Config file name"
        )]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "BOOL",
            help = "Force overwrite to output file if it already exists",
            default_value_t = false
        )]
        force_overwrite: bool,
    },

    /// Generate synthetic RDF data from ShEx or SHACL schemas
    Generate {
        #[arg(
            short = 's',
            long = "schema",
            value_name = "Schema file (ShEx or SHACL)"
        )]
        schema: InputSpec,

        #[arg(
            short = 'f',
            long = "schema-format",
            value_name = "Schema format",
            default_value_t = GenerateSchemaFormat::Auto
        )]
        schema_format: GenerateSchemaFormat,

        #[arg(
            short = 'n',
            long = "entities",
            value_name = "Number of entities to generate",
            default_value_t = 10
        )]
        entity_count: usize,

        #[arg(
            short = 'o',
            long = "output-file",
            value_name = "Output file name, default = terminal"
        )]
        output: Option<PathBuf>,

        #[arg(
            short = 'r',
            long = "result-format",
            value_name = "Output RDF format",
            default_value_t = DataFormat::Turtle
        )]
        result_format: DataFormat,

        #[arg(
            long = "seed",
            value_name = "Random seed for reproducible generation"
        )]
        seed: Option<u64>,

        #[arg(
            short = 'p',
            long = "parallel",
            value_name = "Number of parallel threads"
        )]
        parallel: Option<usize>,

        #[arg(
            short = 'c',
            long = "config",
            value_name = "Configuration file (TOML or JSON)"
        )]
        config: Option<PathBuf>,

        #[arg(
            long = "force-overwrite",
            value_name = "Force overwrite mode",
            default_value_t = false
        )]
        force_overwrite: bool,
    },
}

impl Display for ShowNodeMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShowNodeMode::Outgoing => write!(dest, "outgoing"),
            ShowNodeMode::Incoming => write!(dest, "incoming"),
            ShowNodeMode::Both => write!(dest, "both"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ShExFormat {
    Internal,
    Simple,
    #[default]
    ShExC,
    ShExJ,
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl MimeType for ShExFormat {
    fn mime_type(&self) -> String {
        match self {
            ShExFormat::Internal => "text/turtle".to_string(),
            ShExFormat::Simple => "text/turtle".to_string(),
            ShExFormat::ShExC => "text/shex".to_string(),
            ShExFormat::ShExJ => "application/json".to_string(),
            ShExFormat::Turtle => "text/turtle".to_string(),
            ShExFormat::NTriples => "application/n-triples".to_string(),
            ShExFormat::RDFXML => "application/rdf+xml".to_string(),
            ShExFormat::TriG => "application/trig".to_string(),
            ShExFormat::N3 => "text/n3".to_string(),
            ShExFormat::NQuads => "application/n-quads".to_string(),
        }
    }
}

impl Display for ShExFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShExFormat::Internal => write!(dest, "internal"),
            ShExFormat::Simple => write!(dest, "simple"),
            ShExFormat::ShExC => write!(dest, "shexc"),
            ShExFormat::ShExJ => write!(dest, "shexj"),
            ShExFormat::Turtle => write!(dest, "turtle"),
            ShExFormat::NTriples => write!(dest, "ntriples"),
            ShExFormat::RDFXML => write!(dest, "rdfxml"),
            ShExFormat::TriG => write!(dest, "trig"),
            ShExFormat::N3 => write!(dest, "n3"),
            ShExFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ShapeMapFormat {
    Compact,
    Internal,
}

impl Display for ShapeMapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShapeMapFormat::Compact => write!(dest, "compact"),
            ShapeMapFormat::Internal => write!(dest, "internal"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DataFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultFormat {
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
    Compact,
    Json,
}

impl Display for ResultFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultFormat::Turtle => write!(dest, "turtle"),
            ResultFormat::NTriples => write!(dest, "ntriples"),
            ResultFormat::RDFXML => write!(dest, "rdfxml"),
            ResultFormat::TriG => write!(dest, "trig"),
            ResultFormat::N3 => write!(dest, "n3"),
            ResultFormat::NQuads => write!(dest, "nquads"),
            ResultFormat::Compact => write!(dest, "compact"),
            ResultFormat::Json => write!(dest, "json"),
        }
    }
}

pub trait MimeType {
    fn mime_type(&self) -> String;
}

impl MimeType for DataFormat {
    fn mime_type(&self) -> String {
        match self {
            DataFormat::Turtle => "text/turtle".to_string(),
            DataFormat::NTriples => "application/n-triples".to_string(),
            DataFormat::RDFXML => "application/rdf+xml".to_string(),
            DataFormat::TriG => "application/trig".to_string(),
            DataFormat::N3 => "text/n3".to_string(),
            DataFormat::NQuads => "application/n-quads".to_string(),
        }
    }
}

impl From<DataFormat> for RDFFormat {
    fn from(val: DataFormat) -> Self {
        match val {
            DataFormat::Turtle => RDFFormat::Turtle,
            DataFormat::NTriples => RDFFormat::NTriples,
            DataFormat::RDFXML => RDFFormat::RDFXML,
            DataFormat::TriG => RDFFormat::TriG,
            DataFormat::N3 => RDFFormat::N3,
            DataFormat::NQuads => RDFFormat::NQuads,
        }
    }
}

impl Display for DataFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DataFormat::Turtle => write!(dest, "turtle"),
            DataFormat::NTriples => write!(dest, "ntriples"),
            DataFormat::RDFXML => write!(dest, "rdfxml"),
            DataFormat::TriG => write!(dest, "trig"),
            DataFormat::N3 => write!(dest, "n3"),
            DataFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum ShaclFormat {
    Internal,
    #[default]
    Turtle,
    NTriples,
    RDFXML,
    TriG,
    N3,
    NQuads,
}

impl MimeType for ShaclFormat {
    fn mime_type(&self) -> String {
        match self {
            ShaclFormat::Turtle => "text/turtle".to_string(),
            ShaclFormat::NTriples => "application/n-triples".to_string(),
            ShaclFormat::RDFXML => "application/rdf+xml".to_string(),
            ShaclFormat::TriG => "application/trig".to_string(),
            ShaclFormat::N3 => "text/n3".to_string(),
            ShaclFormat::NQuads => "application/n-quads".to_string(),
            ShaclFormat::Internal => "text/turtle".to_string(),
        }
    }
}

impl Display for ShaclFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ShaclFormat::Internal => write!(dest, "internal"),
            ShaclFormat::Turtle => write!(dest, "turtle"),
            ShaclFormat::NTriples => write!(dest, "NTriples"),
            ShaclFormat::RDFXML => write!(dest, "rdfxml"),
            ShaclFormat::TriG => write!(dest, "trig"),
            ShaclFormat::N3 => write!(dest, "n3"),
            ShaclFormat::NQuads => write!(dest, "nquads"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapFormat {
    CSV,
    XLSX,
    XLSB,
    XLSM,
    XLS,
}

impl Display for DCTapFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapFormat::CSV => write!(dest, "csv"),
            DCTapFormat::XLSX => write!(dest, "xlsx"),
            DCTapFormat::XLSB => write!(dest, "xlsb"),
            DCTapFormat::XLSM => write!(dest, "xlsm"),
            DCTapFormat::XLS => write!(dest, "xls"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum DCTapResultFormat {
    Internal,
    JSON,
}

impl Display for DCTapResultFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            DCTapResultFormat::Internal => write!(dest, "internal"),
            DCTapResultFormat::JSON => write!(dest, "json"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ValidationMode {
    ShEx,
    SHACL,
}

impl Display for ValidationMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ValidationMode::ShEx => write!(dest, "shex"),
            ValidationMode::SHACL => write!(dest, "shacl"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
#[clap(rename_all = "lower")]
pub enum GenerateSchemaFormat {
    #[default]
    Auto,
    ShEx,
    SHACL,
}

impl Display for GenerateSchemaFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            GenerateSchemaFormat::Auto => write!(dest, "auto"),
            GenerateSchemaFormat::ShEx => write!(dest, "shex"),
            GenerateSchemaFormat::SHACL => write!(dest, "shacl"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum InputConvertMode {
    SHACL,
    ShEx,
    DCTAP,
}

impl Display for InputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InputConvertMode::SHACL => write!(dest, "shacl"),
            InputConvertMode::ShEx => write!(dest, "shex"),
            InputConvertMode::DCTAP => write!(dest, "dctap"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum OutputConvertMode {
    SPARQL,
    ShEx,
    UML,
    HTML,
}

impl Display for OutputConvertMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            OutputConvertMode::SPARQL => write!(dest, "sparql"),
            OutputConvertMode::ShEx => write!(dest, "shex"),
            OutputConvertMode::UML => write!(dest, "uml"),
            OutputConvertMode::HTML => write!(dest, "html"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default, Debug)]
#[clap(rename_all = "lower")]
pub enum RDFReaderMode {
    Lax,

    #[default]
    Strict,
}

impl From<RDFReaderMode> for ReaderMode {
    fn from(value: RDFReaderMode) -> Self {
        match value {
            RDFReaderMode::Strict => ReaderMode::Strict,
            RDFReaderMode::Lax => ReaderMode::Lax,
        }
    }
}

impl Display for RDFReaderMode {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match &self {
            RDFReaderMode::Strict => write!(dest, "strict"),
            RDFReaderMode::Lax => write!(dest, "lax"),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
#[clap(rename_all = "lower")]
pub enum ResultServiceFormat {
    Internal,
}

impl Display for ResultServiceFormat {
    fn fmt(&self, dest: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            ResultServiceFormat::Internal => write!(dest, "internal"),
        }
    }
}