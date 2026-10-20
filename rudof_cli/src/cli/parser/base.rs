use crate::cli::parser::{
    CompareArgs, ConfigArgs, ConnectArgs, ConvertArgs, DCTapArgs, DataArgs, DdlArgs, GenerateArgs, LoadArgs,
    MaterializeArgs, McpArgs, NodeArgs, PgschemaArgs, PgschemaValidateArgs, QueryArgs, RdfConfigArgs, ServiceArgs,
    ShaclArgs, ShaclValidateArgs, ShapemapArgs, ShellArgs, ShexArgs, ShexCheckArgs, ShexValidateArgs, SparqlArgs,
    ValidateArgs,
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
    /// Check whether a ShEx schema is well-formed
    ShexCheck(ShexCheckArgs),
    /// Validate RDF using ShEx schemas
    ShexValidate(ShexValidateArgs),
    /// Validate RDF data using SHACL shapes
    ShaclValidate(ShaclValidateArgs),
    /// Show information about RDF data
    Data(DataArgs),
    /// Show information about a node in an RDF Graph
    Node(NodeArgs),
    /// Show information about SHACL shapes
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
    /// Show information about a SPARQL query
    Sparql(SparqlArgs),
    /// Generate synthetic RDF data from ShEx or SHACL schemas
    Generate(GenerateArgs),
    /// Materialize an RDF graph from a ShEx schema and Map semantic-action state
    Materialize(MaterializeArgs),
    /// Validate Property Graph data using PGSchema
    PgschemaValidate(PgschemaValidateArgs),
    /// Dump the effective configuration rudof is using as TOML
    Config(ConfigArgs),
    /// Start an interactive shell (REPL) session
    Shell(ShellArgs),
    /// Connect to (or create) a LadybugDB database and store the connection details
    Connect(ConnectArgs),
    /// Generate property graph DDL (cypher | gql) derived from RDF data
    Ddl(DdlArgs),
    /// Load RDF data into a LadybugDB property graph database
    Load(LoadArgs),
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

    /// Returns the `--qlever-index-dir` override, if set. Only ever `Some`
    /// for [`CommonArgs::All`] — the only variant that carries a backend.
    pub fn qlever_index_dir(&self) -> Option<&PathBuf> {
        match self {
            CommonArgs::All(args) => args.qlever_index_dir.as_ref(),
            _ => None,
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

    /// Choose which backend to load the input into.
    ///
    /// - `memory` (default): parse into an in-process oxrdf::Graph.
    /// - `qlever`: launch a local QLever Docker container and index the input on disk.
    ///   Requires the binary to be built with the `qlever` feature.
    /// - `lbug`: the connected LadybugDB property graph database (see `rudof connect`).
    ///   Accepted here but not yet functional: rudof can derive a property graph from
    ///   RDF (`load`/`ddl`) but cannot yet read one back out as RDF triples.
    /// - `endpoint=<URL_OR_NAME>`: query an external SPARQL endpoint by URL or by
    ///   the name of an endpoint registered in the TOML config. See also the
    ///   `--endpoint` / `-e` shortcut.
    #[arg(
        long = "backend",
        value_name = "BACKEND",
        help = "Backend selection: memory | qlever | lbug | endpoint=<URL_OR_NAME>",
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
        conflicts_with = "backend"
    )]
    pub endpoint: Option<String>,

    /// Minimum interval, in milliseconds, between two requests sent to the
    /// SPARQL endpoint, overriding both the endpoint's own
    /// `min_request_interval_ms` and `[rdf] min_request_interval_ms` from the
    /// TOML config. `0` disables the proactive rate limiter (a `429 Too Many
    /// Requests` response still makes rudof back off). Only meaningful with
    /// `-e`/`--endpoint` or `--backend endpoint=…`.
    #[arg(
        long = "min-request-interval",
        value_name = "MS",
        help = "Minimum milliseconds between two requests to the SPARQL endpoint (-e/--endpoint only); \
                default: the endpoint's configured value, else 1100 for remote hosts and 0 for localhost"
    )]
    pub min_request_interval: Option<u64>,

    /// Maximum number of requests rudof keeps in flight at once to the SPARQL
    /// endpoint when it has several independent ones to make (fetching the
    /// nodes of a shapemap ahead of validation). Requests stay spaced by the
    /// minimum request interval. Overrides `max_concurrent_requests` from the
    /// TOML config. Only meaningful with an endpoint.
    #[arg(
        long = "max-concurrent-requests",
        value_name = "N",
        value_parser = clap::value_parser!(u64).range(1..),
        help = "Maximum requests in flight at once to the SPARQL endpoint (-e/--endpoint only); default 1"
    )]
    pub max_concurrent_requests: Option<u64>,

    /// Maximum number of nodes one SPARQL query fetches the arcs of, when rudof
    /// fetches the nodes of a shapemap ahead of validation. `1` disables
    /// batching. Overrides `max_nodes_per_query` from the TOML config. Only
    /// meaningful with an endpoint.
    #[arg(
        long = "max-nodes-per-query",
        value_name = "N",
        value_parser = clap::value_parser!(u64).range(1..),
        help = "Maximum nodes fetched per SPARQL query when prefetching a shapemap's nodes (-e/--endpoint only); \
                1 disables batching; default 20"
    )]
    pub max_nodes_per_query: Option<u64>,

    /// Directory for the QLever on-disk index, overriding `[rdf.qlever]
    /// index_dir` from the TOML config.
    ///
    /// Only meaningful with `--backend qlever`. Naming a directory explicitly
    /// is how you find out (and share) where an index lives: pass the same
    /// directory again — from this machine or another rudof session pointed
    /// at the same path (e.g. a shared/mounted volume) — and the idempotent
    /// build step is skipped, going straight to serving. When unset, rudof
    /// resolves a per-input fingerprinted path under the platform cache dir
    /// and logs it at startup.
    #[arg(
        long = "qlever-index-dir",
        value_name = "DIR",
        help = "Directory for the QLever on-disk index (--backend qlever only); enables sharing/reuse across sessions"
    )]
    pub qlever_index_dir: Option<PathBuf>,
}

impl CommonArgsAll {
    /// The endpoint request limits given on the command line.
    pub fn endpoint_limits(&self) -> rudof_rdf::rdf_core::EndpointLimits {
        rudof_rdf::rdf_core::EndpointLimits::default()
            .with_min_request_interval_ms(self.min_request_interval)
            .with_max_concurrent_requests(self.max_concurrent_requests.map(|n| n as usize))
            .with_max_nodes_per_query(self.max_nodes_per_query.map(|n| n as usize))
    }
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

#[cfg(test)]
mod tests {
    use super::CommonArgsAll;
    use clap::Parser;

    /// Standalone wrapper so `CommonArgsAll` can be parsed on its own, the
    /// same way it's `#[command(flatten)]`-ed into every real subcommand.
    #[derive(Debug, Parser)]
    struct TestCli {
        #[command(flatten)]
        common: CommonArgsAll,
    }

    #[test]
    fn qlever_index_dir_is_unset_by_default() {
        let cli = TestCli::parse_from(["test", "--backend", "qlever"]);
        assert_eq!(cli.common.qlever_index_dir, None);
    }

    #[test]
    fn qlever_index_dir_parses_alongside_backend() {
        let cli = TestCli::parse_from([
            "test",
            "--backend",
            "qlever",
            "--qlever-index-dir",
            "/shared/qlever-index",
        ]);
        assert_eq!(
            cli.common.qlever_index_dir,
            Some(std::path::PathBuf::from("/shared/qlever-index"))
        );
    }

    #[test]
    fn min_request_interval_parses_alongside_endpoint() {
        let cli = TestCli::parse_from(["test", "-e", "http://localhost/sparql", "--min-request-interval", "0"]);
        assert_eq!(cli.common.min_request_interval, Some(0));
        let cli = TestCli::parse_from(["test", "-e", "wikidata"]);
        assert_eq!(cli.common.min_request_interval, None);
    }

    #[test]
    fn endpoint_limits_collect_the_endpoint_flags() {
        let cli = TestCli::parse_from([
            "test",
            "-e",
            "http://localhost/sparql",
            "--max-concurrent-requests",
            "4",
            "--max-nodes-per-query",
            "50",
        ]);
        let limits = cli.common.endpoint_limits();
        assert_eq!(limits.min_request_interval_ms, None);
        assert_eq!(limits.max_concurrent_requests, Some(4));
        assert_eq!(limits.max_nodes_per_query, Some(50));
        assert!(TestCli::try_parse_from(["test", "--max-nodes-per-query", "0"]).is_err());
    }
}
