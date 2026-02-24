use crate::cli::wrappers::{
    DCTapFormatCli, DCTapResultFormatCli, DataFormatCli, GenerateSchemaFormatCli, InputCompareFormatCli,
    InputCompareModeCli, InputConvertFormatCli, InputConvertModeCli, OutputConvertFormatCli, OutputConvertModeCli,
    PgSchemaFormatCli, QueryTypeCli, RDFReaderModeCli, RdfConfigFormatCli, RdfConfigResultFormatCli,
    ResultCompareFormatCli, ResultDataFormatCli, ResultQueryFormatCli, ResultServiceFormatCli,
    ResultShExValidationFormatCli, ResultShaclValidationFormatCli, ResultValidationFormatCli, ShExFormatCli,
    ShaclFormatCli, ShaclValidationModeCli, ShapeMapFormatCli, ShowNodeModeCli, SortByResultShapeMapCli,
    SortByShaclValidationReportCli, SortByValidateCli, ValidationModeCli, PgSchemaResultFormatCli,
};
use clap::{Args, Parser, Subcommand};
use rudof_lib::{InputSpec, IriS};
use rudof_mcp::server::TransportType;
use std::path::PathBuf;

/// Rudof
///
/// This CLI allows for the validation, conversion, and generation of RDF
/// and Property Graphs using various schema languages like ShEx and SHACL.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(
    arg_required_else_help = true,
    long_about = None // Automatically uses the Doc Comment above
)]
pub struct Cli {
    /// Main command to execute
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Increase logging verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Export rudof as an MCP server
    Mcp(McpArgs),
    /// Show information about ShEx ShapeMaps
    Shapemap(ShapemapArgs),
    /// Show information about ShEx schemas
    Shex(ShexArgs),
    /// Show information about Property Graph Schemas
    Pgschema(PgschemaArgs),
    /// Validate RDF data using ShEx or SHACL
    Validate(ValidateArgs),
    /// Validate RDF using ShEx schemas
    ShexValidate(ShexValidateArgs),
    /// Validate RDF data using SHACL shapes
    ShaclValidate(ShaclValidateArgs),
    /// Show information about RDF data
    Data(DataArgs),
    /// Show information about a node in an RDF Graph
    Node(NodeArgs),
    /// Show information about SHACL shapes
    /// The SHACL schema can be passed through the data options or the optional schema options to provide an interface similar 
    /// to Shacl-validate
    Shacl(ShaclArgs),
    // Show information and process DCTAP files
    #[command(name = "dctap")]
    DCTap(DCTapArgs),
    // Convert between different Data modeling technologies
    Convert(ConvertArgs),
    /// Compare two shapes (which can be in different formats)
    Compare(CompareArgs),
    /// Show information about rdf config
    RdfConfig(RdfConfigArgs),
    /// Show information about SPARQL service
    Service(ServiceArgs),
    /// Run SPARQL queries
    Query(QueryArgs),
    /// Generate synthetic RDF data from ShEx or SHACL schemas
    Generate(GenerateArgs),
    /// Validate Property Graph data using PGSchema
    PgSchemaValidate(PgSchemaValidateArgs),
}

// ============================================================================
// Command Args
// ============================================================================

/// Arguments shared across multiple commands.
///
/// Using `flatten` ensures a consistent interface for output
/// and configuration management.
#[derive(Debug, Clone)]
pub enum CommonArgs {
    /// Contains all common arguments including config, output, and force overwrite.
    All(CommonArgsAll),
    /// Contains only output and force overwrite arguments.            
    OutputForceOverWrite(CommonArgsOutputForceOverWrite),
    /// Represents the absence of common arguments.
    None,
}

impl CommonArgs {
    /// Returns the config file path if it exists.
    pub fn config(&self) -> Option<&PathBuf> {
        match self {
            CommonArgs::All(args) => args.config.as_ref(),
            CommonArgs::OutputForceOverWrite(_) => None,
            CommonArgs::None => None,
        }
    }

    /// Returns the output file path if it exists.
    pub fn output(&self) -> Option<&PathBuf> {
        match self {
            CommonArgs::All(args) => args.output.as_ref(),
            CommonArgs::OutputForceOverWrite(args) => args.output.as_ref(),
            CommonArgs::None => None,
        }
    }

    /// Returns whether the force-overwrite flag is enabled.
    pub fn force_overwrite(&self) -> bool {
        match self {
            CommonArgs::All(args) => args.force_overwrite,
            CommonArgs::OutputForceOverWrite(args) => args.force_overwrite,
            CommonArgs::None => false,
        }
    }
}

/// Full set of common arguments for commands that support config and output.
#[derive(Debug, Clone, Args)]
pub struct CommonArgsAll {
    #[arg(short = 'c', long = "config-file", value_name = "FILE", help = "Config file name")]
    pub config: Option<PathBuf>,

    #[arg(
        short = 'o',
        long = "output-file",
        value_name = "FILE",
        help = "Output file name, default = terminal"
    )]
    pub output: Option<PathBuf>,

    #[arg(
        long = "force-overwrite",
        value_name = "BOOL",
        help = "Force overwrite to output file if it already exists",
        default_value_t = false
    )]
    pub force_overwrite: bool,
}

/// Partial common arguments for commands that only handle output and overwriting.
#[derive(Debug, Clone, Args)]
pub struct CommonArgsOutputForceOverWrite {
    #[arg(
        short = 'o',
        long = "output-file",
        value_name = "FILE",
        help = "Output file name, default = terminal"
    )]
    pub output: Option<PathBuf>,

    #[arg(
        long = "force-overwrite",
        value_name = "BOOL",
        help = "Force overwrite to output file if it already exists",
        default_value_t = false
    )]
    pub force_overwrite: bool,
}

/// Arguments for the `mcp` command
#[derive(Debug, Clone, Args)]
pub struct McpArgs {
    #[arg(
        short = 't',
        long = "transport",
        value_name = "TRANSPORT",
        ignore_case = true,
        help = "Transport type: stdio (for CLI/IDE) or streamable-http (for web clients)",
        default_value_t = TransportType::Stdio
    )]
    pub transport: TransportType,

    #[arg(
        short = 'b',
        long = "bind",
        value_name = "ADDRESS",
        help = "Bind address for HTTP transport. Examples: 127.0.0.1 (localhost IPv4), \
              0.0.0.0 (all IPv4 interfaces), ::1 (localhost IPv6), :: (all IPv6 interfaces). \
              Default: 127.0.0.1 for security",
        default_value = "127.0.0.1"
    )]
    pub bind_address: String,

    #[arg(
        short = 'p',
        long = "port",
        value_name = "PORT",
        help = "Port number for HTTP transport (only used with http-sse transport)",
        default_value_t = 8000
    )]
    pub port: u16,

    #[arg(
        short = 'r',
        long = "route",
        value_name = "PATH",
        help = "Route path for HTTP transport (only used with http-sse transport)",
        default_value = "/rudof"
    )]
    pub route_path: String,

    #[arg(
        short = 'n',
        long = "allowed-network",
        value_name = "CIDR",
        help = "Allowed IP network in CIDR notation (only used with http-sse transport). \
              Can be specified multiple times to allow multiple networks. \
              Examples: 127.0.0.1, 192.168.1.0/24, 10.0.0.0/8, ::1. \
              If not specified, defaults to localhost only (127.0.0.0/8 and ::1/128)",
        num_args = 0.. // allows multiple vales
    )]
    pub allowed_networks: Vec<String>,
}

/// Arguments for the `shapemap` command
#[derive(Debug, Clone, Args)]
pub struct ShapemapArgs {
    #[arg(
        short = 'm',
        long = "shapemap",
        value_name = "INPUT",
        help = "ShapeMap (FILE, URI or - for stdin)"
    )]
    pub shapemap: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format, default = compact", 
        default_value_t = ShapeMapFormatCli::Compact
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Result shapemap format, default = compact",
        default_value_t = ShapeMapFormatCli::Compact
    )]
    pub result_shapemap_format: ShapeMapFormatCli,

    #[command(flatten)]
    pub common: CommonArgsOutputForceOverWrite,
}

/// Arguments for the `shex` command
#[derive(Debug, Clone, Args)]
pub struct ShexArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema, FILE, URI or - for stdin"
    )]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result schema format",
        default_value_t = ShExFormatCli::ShExJ
    )]
    pub result_schema_format: ShExFormatCli,

    #[arg(short = 'l', long = "shape-label", value_name = "LABEL", help = "shape label")]
    pub shape: Option<String>,

    #[arg(short = 't', value_name = "BOOL", help = "Show processing time", long = "show-time")]
    pub show_time: Option<bool>,

    #[arg(long = "show-schema", value_name = "BOOL", help = "Show schema")]
    pub show_schema: Option<bool>,

    #[arg(long = "statistics", value_name = "BOOL", help = "Show statistics about the schema")]
    pub show_statistics: Option<bool>,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<IriS>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode (strict or lax)",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        long = "show-dependencies",
        value_name = "BOOL",
        help = "Show dependencies between shapes"
    )]
    pub show_dependencies: Option<bool>,

    #[arg(
        long = "compile",
        value_name = "BOOL",
        help = "Compile Schema to Internal representation"
    )]
    pub compile: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `pgschema` command
#[derive(Debug, Clone, Args)]
pub struct PgschemaArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema, FILE, URI or - for stdin"
    )]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "PGSchema format",
        default_value_t = PgSchemaFormatCli::PgSchemaC,
        value_enum
    )]
    pub schema_format: PgSchemaFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result schema format",
        default_value_t = PgSchemaFormatCli::PgSchemaC,
        value_enum
    )]
    pub result_schema_format: PgSchemaFormatCli,

    #[arg(short = 't', value_name = "BOOL", help = "Show processing time", long = "show-time")]
    pub show_time: Option<bool>,

    #[arg(long = "show-schema", value_name = "BOOL", help = "Show schema")]
    pub show_schema: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `validate` command
#[derive(Debug, Clone, Args)]
pub struct ValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(short = 'M', long = "mode",
        value_name = "MODE",
        ignore_case = true,
        help = "Validation mode (ShEx or SHACL)",
        default_value_t = ValidationModeCli::ShEx
    )]
    pub validation_mode: ValidationModeCli,

    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema used for validatio, FILE, URI or - for stdin"
    )]
    pub schema: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Schema format"
    )]
    pub schema_format: Option<ShExFormatCli>,

    #[arg(
        short = 'm',
        long = "shapemap",
        value_name = "INPUT",
        help = "ShapeMap used for validation, FILE, URI or - for stdin"
    )]
    pub shapemap: Option<InputSpec>,

    #[arg(
        long = "shapemap-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format",
        default_value_t = ShapeMapFormatCli::Compact,
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(long = "base-data", value_name = "IRI", help = "Base IRI for data")]
    pub base_data: Option<IriS>,

    #[arg(long = "base-schema", value_name = "IRI", help = "Base IRI for Schema")]
    pub base_schema: Option<IriS>,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by (default = node)",
        default_value_t = SortByValidateCli::Node,
        value_enum
    )]
    pub sort_by: SortByValidateCli,

    #[arg(short = 'n', long = "node", value_name = "NODE", help = "Node to validate")]
    pub node: Option<String>,

    #[arg(
        short = 'l',
        long = "shape-label",
        value_name = "LABEL",
        help = "shape label (default = START)",
        group = "node_shape"
    )]
    pub shape: Option<String>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF Data format (default = turtle)",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "ENDPOINT",
        help = "Endpoint with RDF data"
    )]
    pub endpoint: Option<String>,

    #[arg(
        long = "max-steps",
        value_name = "NUMBER",
        help = "max steps to run during validation",
        default_value_t = 100
    )]
    pub max_steps: usize,

    #[arg(
        short = 'S',
        long = "shacl-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "SHACL validation mode (default = native)",
        default_value_t = ShaclValidationModeCli::Native,
        value_enum
    )]
    pub shacl_validation_mode: ShaclValidationModeCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE", help = "RDF Reader mode",
        ignore_case = true,
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT", help = "Ouput result format, default = compact",
        default_value_t = ResultValidationFormatCli::Compact
    )]
    pub result_format: ResultValidationFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `shex-validate` command
#[derive(Debug, Clone, Args)]
pub struct ShexValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Schema file name, URI or - (for stdin)"
    )]
    pub schema: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "ShEx Schema format"
    )]
    pub schema_format: Option<ShExFormatCli>,

    #[arg(short = 'm', long = "shapemap", value_name = "INPUT", help = "ShapeMap")]
    pub shapemap: Option<InputSpec>,

    #[arg(
        long = "shapemap-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format",
        default_value_t = ShapeMapFormatCli::Compact,
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(short = 'n', long = "node", value_name = "NODE", help = "Node to validate")]
    pub node: Option<String>,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by (default = node)",
        default_value_t = SortByResultShapeMapCli::Node,
        value_enum
    )]
    pub sort_by: SortByResultShapeMapCli,

    #[arg(
        short = 'l',
        long = "shape-label",
        value_name = "LABEL",
        help = "shape label (default = START)",
        group = "node_shape"
    )]
    pub shape: Option<String>,

    #[arg(
        short = 't',
        long = "data-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        long = "base-schema",
        value_name = "IRI",
        help = "Base Schema (used to resolve relative IRIs in Schema)"
    )]
    pub base_schema: Option<IriS>,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base RDF Data IRI (used to resolve relative IRIs in RDF data)"
    )]
    pub base_data: Option<IriS>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "NAME",
        help = "Endpoint with RDF data (name or URL)"
    )]
    pub endpoint: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Ouput result format",
        default_value_t = ResultShExValidationFormatCli::Details
    )]
    pub result_format: ResultShExValidationFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `shacl-validate` command
#[derive(Debug, Clone, Args)]
pub struct ShaclValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help= "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs in RDF data)"
    )]
    pub base_data: Option<IriS>,

    /// RDF Reader mode
    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        short = 's',
        long = "shapes",
        value_name = "INPUT",
        help = "Shapes graph: file, URI or -, if not set, it assumes the shapes come from the data"
    )]
    pub shapes: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "shapes-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Shapes file format"
    )]
    pub shapes_format: Option<ShaclFormatCli>,

    #[arg(
        long = "base-shapes",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs in Shapes)"
    )]
    pub base_shapes: Option<IriS>,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "ENDPOINT",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    /// Execution mode
    #[arg(
        short = 'm',
        long = "mode",
        value_name = "MODE",
        ignore_case = true,
        help = "Execution mode",
        default_value_t = ShaclValidationModeCli::Native,
        value_enum
    )]
    pub mode: ShaclValidationModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Ouput result format",
        default_value_t = ResultShaclValidationFormatCli::Details
    )]
    pub result_format: ResultShaclValidationFormatCli,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by",
        default_value_t = SortByShaclValidationReportCli::Severity,
        value_enum
    )]
    pub sort_by: SortByShaclValidationReportCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `shacl-validate` command
#[derive(Debug, Clone, Args)]
pub struct DataArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<IriS>,

    /// RDF Reader mode
    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Ouput result format",
        default_value_t = ResultDataFormatCli::Turtle
    )]
    pub result_format: ResultDataFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `node` command
#[derive(Debug, Clone, Args)]
pub struct NodeArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 'n',
        long = "node",
        value_name = "Node",
        help = "Node to show information (can be a URI or prefixed name)"
    )]
    pub node: String,

    #[arg(
        short = 't',
        long = "data-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    #[arg(short = 'b', long = "base", value_name = "IRI", help = "Base IRI")]
    pub base: Option<IriS>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        short = 'm',
        long = "show-node-mode",
        ignore_case = true,
        value_name = "MODE",
        help = "Mode used to show the node information",
        default_value_t = ShowNodeModeCli::Outgoing
    )]
    pub show_node_mode: ShowNodeModeCli,

    #[arg(long = "show hyperlinks", help = "Show hyperlinks in the output")]
    pub show_hyperlinks: bool,

    #[arg(
        short = 'p',
        long = "predicates",
        value_name = "PREDICATES",
        help = "List of predicates to show"
    )]
    pub predicates: Vec<String>,

    #[arg(
        short = 'd',
        long = "depth",
        value_name = "NUMBER",
        help = "outgoing number of levels, default = 1",
        default_value_t = 1
    )]
    pub depth: usize,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `shacl` command
#[derive(Debug, Clone, Args)]
pub struct ShaclArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    #[arg(
        short = 's',
        long = "shapes",
        value_name = "INPUT",
        help = "Shapes graph: File, URI or - for stdin, if not set, it assumes the shapes come from the data"
    )]
    pub shapes: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "shapes-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Shapes file format"
    )]
    pub shapes_format: Option<ShaclFormatCli>,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base RDF Data (used to resolve relative IRIs in RDF data)"
    )]
    pub base_data: Option<IriS>,

    #[arg(
        long = "base-shapes",
        value_name = "IRI",
        help = "Base RDF Data (used to resolve relative IRIs in Shapes)"
    )]
    pub base_shapes: Option<IriS>,

    #[arg(
        short = 'r',
        long = "result-shapes-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result shapes format",
        default_value_t = ShaclFormatCli::Internal
    )]
    pub result_shapes_format: ShaclFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `dctap` command
#[derive(Debug, Clone, Args)]
pub struct DCTapArgs {
    #[arg(short = 's', long = "source-file", value_name = "FILE", help = "DCTap source file")]
    pub file: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "DCTap file format",
        default_value_t = DCTapFormatCli::Csv
    )]
    pub format: DCTapFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Ouput results format",
        default_value_t = DCTapResultFormatCli::Internal
    )]
    pub result_format: DCTapResultFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `convert` command
#[derive(Debug, Clone, Args)]
pub struct ConvertArgs {
    #[arg(
        short = 'm',
        long = "input-mode",
        ignore_case = true,
        value_name = "MODE",
        help = "Input mode"
    )]
    input_mode: InputConvertModeCli,

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
        ignore_case = true,
        value_name = "FORMAT",
        help = "Input file format",
        default_value_t = InputConvertFormatCli::ShExC
    )]
    format: InputConvertFormatCli,

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
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result format",
        default_value_t = OutputConvertFormatCli::Default
    )]
    result_format: OutputConvertFormatCli,

    #[arg(short = 't', long = "target-folder", value_name = "FOLDER", help = "Target folder")]
    target_folder: Option<PathBuf>,

    #[arg(
        short = 'e',
        long = "templates-folder",
        ignore_case = true,
        value_name = "FOLDER",
        help = "Templates folder"
    )]
    template_folder: Option<PathBuf>,

    #[arg(
        short = 'l',
        long = "shape-label",
        value_name = "LABEL",
        help = "shape label (default = START)"
    )]
    shape: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    reader_mode: RDFReaderModeCli,

    #[arg(
        short = 'x',
        long = "export-mode",
        ignore_case = true,
        value_name = "MODE",
        help = "Result mode for conversion"
    )]
    output_mode: OutputConvertModeCli,

    #[arg(long = "show-time", help = "Show processing time")]
    show_time: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `compare` command
#[derive(Debug, Clone, Args)]
pub struct CompareArgs {
    #[arg(long = "mode1",
        value_name = "MODE",
        ignore_case = true,
        help = "Input mode first schema",
        default_value_t = InputCompareModeCli::ShEx
    )]
    pub input_mode1: InputCompareModeCli,

    #[arg(
        long = "mode2",
        value_name = "MODE",
        ignore_case = true,
        help = "Input mode second schema",
        default_value_t = InputCompareModeCli::ShEx
    )]
    pub input_mode2: InputCompareModeCli,

    #[arg(long = "schema1", value_name = "INPUT", help = "Schema 1 (URI, file or - for stdin)")]
    pub schema1: InputSpec,

    #[arg(long = "schema2", value_name = "INPUT", help = "Schema 2 (URI, file or - for stdin)")]
    pub schema2: InputSpec,

    #[arg(
        long = "format1",
        value_name = "FORMAT",
        ignore_case = true,
        help = "File format 1",
        default_value_t = InputCompareFormatCli::ShExC
    )]
    pub format1: InputCompareFormatCli,

    #[arg(
        long = "format2",
        value_name = "FORMAT",
        ignore_case = true,
        help = "File format 2",
        default_value_t = InputCompareFormatCli::ShExC
    )]
    pub format2: InputCompareFormatCli,

    #[arg(long = "base1", value_name = "IRI", help = "Base IRI for 1st Schema")]
    pub base1: Option<IriS>,

    #[arg(long = "base2", value_name = "IRI", help = "Base IRI for 2nd Schema")]
    pub base2: Option<IriS>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Result format",
        default_value_t = ResultCompareFormatCli::Internal
    )]
    pub result_format: ResultCompareFormatCli,

    #[arg(short = 't', long = "target-folder", value_name = "FOLDER", help = "Target folder")]
    pub target_folder: Option<PathBuf>,

    #[arg(long = "shape1", value_name = "LABEL", help = "shape1 (default = START)")]
    pub shape1: Option<String>,

    #[arg(long = "shape2", value_name = "LABEL", help = "shape2 (default = START)")]
    pub shape2: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(long = "show-time", help = "Show processing time")]
    pub show_time: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `rdf-config` command
#[derive(Debug, Clone, Args)]
pub struct RdfConfigArgs {
    #[arg(
        short = 's',
        long = "source-file",
        value_name = "INPUT",
        help = "Source file name (URI, file or - for stdin)"
    )]
    pub input: InputSpec,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Output result rdf-config format",
        default_value_t = RdfConfigResultFormatCli::Internal
    )]
    pub result_format: RdfConfigResultFormatCli,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "rdf-config format",
        default_value_t = RdfConfigFormatCli::Yaml
    )]
    pub format: RdfConfigFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `service` command
#[derive(Debug, Clone, Args)]
pub struct ServiceArgs {
    #[arg(short = 's', long = "service", value_name = "URL", help = "SPARQL service URL")]
    pub service: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "SPARQL service format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub service_format: DataFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Output result service format",
        default_value_t = ResultServiceFormatCli::Json
    )]
    pub result_service_format: ResultServiceFormatCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `query` command
#[derive(Debug, Clone, Args)]
pub struct QueryArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "RDF Data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'b',
        long = "base",
        value_name = "IRI",
        help = "Base IRI (used to resolve relative IRIs in RDF data)"
    )]
    pub base: Option<IriS>,

    #[arg(long = "query-type",
        value_name = "TYPE",
        ignore_case = true,
        help = "Query type (SELECT, ASK, CONSTRUCT, DESCRIBE)",
        default_value_t = QueryTypeCli::Select,
        value_enum
    )]
    pub query_type: QueryTypeCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = RDFReaderModeCli::Strict,
            value_enum
    )]
    pub reader_mode: RDFReaderModeCli,

    #[arg(short = 'q', long = "query", value_name = "INPUT", help = "SPARQL query")]
    pub query: InputSpec,

    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "Endpoint",
        help = "Endpoint with RDF data (URL or name)"
    )]
    pub endpoint: Option<String>,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result query format",
        default_value_t = ResultQueryFormatCli::Internal
    )]
    pub result_query_format: ResultQueryFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `generate` command
#[derive(Debug, Clone, Args)]
pub struct GenerateArgs {
    #[arg(short = 's', long = "schema", value_name = "Schema file (ShEx or SHACL)")]
    pub schema: InputSpec,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "Schema format",
        default_value_t = GenerateSchemaFormatCli::Auto
    )]
    pub schema_format: GenerateSchemaFormatCli,

    #[arg(
        short = 'n',
        long = "entities",
        value_name = "Number of entities to generate",
        default_value_t = 10
    )]
    pub entity_count: usize,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "Output RDF format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub result_format: DataFormatCli,

    #[arg(long = "seed", value_name = "Random seed for reproducible generation")]
    pub seed: Option<u64>,

    #[arg(short = 'p', long = "parallel", value_name = "Number of parallel threads")]
    pub parallel: Option<usize>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `pgschema-validate` command
#[derive(Debug, Clone, Args)]
pub struct PgSchemaValidateArgs {
    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "PGSchema file, URI or - (for stdin)"
    )]
    pub schema: Option<InputSpec>,

    #[clap(value_parser = clap::value_parser!(InputSpec))]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Property Graph data format",
        default_value_t = DataFormatCli::Pg
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 'm',
        long = "shapemap",
        value_name = "INPUT",
        help = "ShapeMap used for validation, FILE, URI or - for stdin"
    )]
    pub shapemap: Option<InputSpec>,

    #[arg(
        long = "shapemap-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "ShapeMap format",
        default_value_t = ShapeMapFormatCli::Compact,
    )]
    pub shapemap_format: ShapeMapFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Output result format",
        default_value_t = PgSchemaResultFormatCli::Compact
    )]
    pub result_validation_format: PgSchemaResultFormatCli,

    #[command(flatten)]
    pub common: CommonArgsOutputForceOverWrite,
}