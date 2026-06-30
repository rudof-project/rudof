use crate::cli::parser::{
    CompareArgs, CompletionArgs, ConvertArgs, DCTapArgs, DataArgs, GenerateArgs, MaterializeArgs, McpArgs, NodeArgs,
    PgschemaArgs, PgschemaValidateArgs, QueryArgs, RdfConfigArgs, ServiceArgs, ShaclArgs, ShaclValidateArgs,
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
    /// Materialize an RDF graph from a ShEx schema and Map semantic-action state
    Materialize(MaterializeArgs),
    /// Validate Property Graph data using PGSchema
    PgschemaValidate(PgschemaValidateArgs),
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
    /// Config, output, force-overwrite, and backend.
    All(CommonArgsAll),
    /// Config, output, and force-overwrite (no backend).
    NoBackend(CommonArgsNoBackend),
    /// Output and force-overwrite only.
    OutputForceOverWrite(CommonArgsOutputForceOverWrite),
    /// Represents the absence of common arguments.
    None,
}

impl CommonArgs {
    /// Returns the config file path if it exists.
    pub fn config(&self) -> Option<&PathBuf> {
        match self {
            CommonArgs::All(args) => args.config.as_ref(),
            CommonArgs::NoBackend(args) => args.config.as_ref(),
            CommonArgs::OutputForceOverWrite(_) => None,
            CommonArgs::None => None,
        }
    }

    /// Returns the output file path if it exists.
    pub fn output(&self) -> Option<&PathBuf> {
        match self {
            CommonArgs::All(args) => args.output.as_ref(),
            CommonArgs::NoBackend(args) => args.output.as_ref(),
            CommonArgs::OutputForceOverWrite(args) => args.output.as_ref(),
            CommonArgs::None => None,
        }
    }

    /// Returns whether the force-overwrite flag is enabled.
    pub fn force_overwrite(&self) -> bool {
        match self {
            CommonArgs::All(args) => args.force_overwrite,
            CommonArgs::NoBackend(args) => args.force_overwrite,
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

    /// Choose which RDF data backend to load the input into.
    ///
    /// - `memory` (default): parse into an in-process oxrdf::Graph.
    /// - `qlever`: launch a local QLever Docker container and index the input on disk.
    ///   Requires the binary to be built with the `qlever` feature.
    /// - `endpoint=<URL_OR_NAME>`: query an external SPARQL endpoint by URL or by
    ///   the name of an endpoint registered in the TOML config. See also the
    ///   `--endpoint` / `-e` shortcut.
    #[arg(
        long = "backend",
        value_name = "BACKEND",
        help = "RDF data backend selection: memory | qlever | endpoint=<URL_OR_NAME>",
        value_parser = clap::builder::ValueParser::new(|s: &str| {
            use std::str::FromStr;
            crate::cli::wrappers::BackendKindCli::from_str(s)
        }),
        conflicts_with = "endpoint",
    )]
    pub backend: Option<crate::cli::wrappers::BackendKindCli>,

    /// Shortcut for `--backend endpoint=<URL_OR_NAME>`.
    ///
    /// Accepts either a full SPARQL endpoint URL or the name of an endpoint
    /// registered in the TOML config. Mutually exclusive with `--backend`.
    #[arg(
        short = 'e',
        long = "endpoint",
        value_name = "URL_OR_NAME",
        help = "SPARQL endpoint URL or named endpoint (shortcut for --backend endpoint=…)",
        conflicts_with = "backend",
    )]
    pub endpoint: Option<String>,
}

/// Common arguments for commands that need config and output but not a backend.
#[derive(Debug, Clone, Args)]
pub struct CommonArgsNoBackend {
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
