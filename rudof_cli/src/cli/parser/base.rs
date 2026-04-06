use crate::cli::parser::{
    CompareArgs, CompletionArgs, ConvertArgs, DCTapArgs, DataArgs, GenerateArgs, McpArgs, NodeArgs,
    PgSchemaValidateArgs, PgschemaArgs, QueryArgs, RdfConfigArgs, ServiceArgs, ShaclArgs, ShaclValidateArgs,
    ShapemapArgs, ShexArgs, ShexValidateArgs, ValidateArgs,
};
use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Rudof
///
/// This CLI allows for the validation, conversion, and generation of RDF
/// and Property Graphs using various schema languages like ShEx and SHACL.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(
    name = "rudof",
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
    /// Generates a shell completion script for the specified shell
    Completion(CompletionArgs),
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
