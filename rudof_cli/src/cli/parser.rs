use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;
use rudof_mcp::server::TransportType;
use rudof_lib::{
    InputSpec, IriS,
};
use crate::cli::{
    formats::{
        PgSchemaFormatCli, ValidationModeCli, SortByValidateCli,
    },
    wrappers::{
        ShapeMapFormatCli, ShExFormatCli, ReaderModeCli, DataFormatCli, ShaclValidationModeCli,
        ResultValidationFormatCli,
    }
};

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
    #[arg(
        short = 'c', 
        long = "config-file", 
        value_name = "FILE", 
        help = "Config file name"
    )]
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
    schema: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Schema format (ShExC, ShExJ, Turtle, ...), default = ShExC",
        default_value_t = ShExFormatCli::ShExC
    )]
    schema_format: ShExFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result schema format",
        default_value_t = ShExFormatCli::ShExJ
    )]
    result_schema_format: ShExFormatCli,

    #[arg(
        short = 'l', 
        long = "shape-label", 
        value_name = "LABEL", 
        help = "shape label"
    )]
    shape: Option<String>,

    #[arg(
        short = 't', 
        value_name = "BOOL", 
        help = "Show processing time", 
        long = "show-time"
    )]
    show_time: Option<bool>,

    #[arg(
        long = "show-schema", 
        value_name = "BOOL", 
        help = "Show schema"
    )]
    show_schema: Option<bool>,

    #[arg(
        long = "statistics", 
        value_name = "BOOL", 
        help = "Show statistics about the schema"
    )]
    show_statistics: Option<bool>,

    #[arg(
        short = 'b', 
        long = "base", 
        value_name = "IRI", 
        help = "Base IRI"
    )]
    base: Option<IriS>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode (strict or lax)",
        default_value_t = ReaderModeCli::Strict,
        value_enum
    )]
    reader_mode: ReaderModeCli,

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
    schema: InputSpec,

    #[arg(
        short = 'f',
        long = "format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "PGSchema format",
        default_value_t = PgSchemaFormatCli::PgSchemaC,
        value_enum
    )]
    schema_format: PgSchemaFormatCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Result schema format",
        default_value_t = PgSchemaFormatCli::PgSchemaC,
        value_enum
    )]
    result_schema_format: PgSchemaFormatCli,

    #[arg(
        short = 't', 
        value_name = "BOOL", 
        help = "Show processing time", 
        long = "show-time"
    )]
    show_time: Option<bool>,

    #[arg(
        long = "show-schema", 
        value_name = "BOOL", 
        help = "Show schema"
    )]
    show_schema: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsAll,
}

/// Arguments for the `validate` command
#[derive(Debug, Clone, Args)]
pub struct ValidateArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec))]
    data: Vec<InputSpec>,

    #[arg(short = 'M', long = "mode",
        value_name = "MODE",
        ignore_case = true,
        help = "Validation mode (ShEx or SHACL)",
        default_value_t = ValidationModeCli::ShEx
    )]
    validation_mode: ValidationModeCli,

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
        ignore_case = true,
        value_name = "FORMAT",
        help = "Schema format"
    )]
    schema_format: Option<ShExFormatCli>,

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
        ignore_case = true,
        help = "ShapeMap format",
        default_value_t = ShapeMapFormatCli::Compact,
    )]
    shapemap_format: ShapeMapFormatCli,

    #[arg(
        long = "base-data", 
        value_name = "IRI", 
        help = "Base IRI for data"
    )]
    base_data: Option<IriS>,

    #[arg(
        long = "base-schema", 
        value_name = "IRI", 
        help = "Base IRI for Schema"
    )]
    base_schema: Option<IriS>,

    #[arg(
        long = "sort_by",
        value_name = "SORT_MODE",
        ignore_case = true,
        help = "Sort result by (default = node)",
        default_value_t = SortByValidateCli::Node,
        value_enum
    )]
    sort_by: SortByValidateCli,

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
        ignore_case = true,
        help = "RDF Data format (default = turtle)",
        default_value_t = DataFormatCli::Turtle
    )]
    data_format: DataFormatCli,

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

    #[arg(
        short = 'S',
        long = "shacl-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "SHACL validation mode (default = native)",
        default_value_t = ShaclValidationModeCli::Native,
        value_enum
    )]
    shacl_validation_mode: ShaclValidationModeCli,

    #[arg(
        long = "reader-mode",
        value_name = "MODE", help = "RDF Reader mode",
        ignore_case = true,
        default_value_t = ReaderModeCli::Strict,
        value_enum
    )]
    reader_mode: ReaderModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        ignore_case = true,
        value_name = "FORMAT", help = "Ouput result format, default = compact",
        default_value_t = ResultValidationFormatCli::Compact
    )]
    result_format: ResultValidationFormatCli,

    #[command(flatten)]
    pub common: CommonArgsAll,
}