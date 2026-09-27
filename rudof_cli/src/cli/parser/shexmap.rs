use crate::cli::parser::CommonArgsNoBackend;
use crate::cli::wrappers::{DataFormatCli, DataReaderModeCli, ResultDataFormatCli, ShExFormatCli};
use clap::Args;
use rudof_lib::formats::InputSpec;
use std::path::PathBuf;

/// Arguments for the `shexmap` command
#[derive(Debug, Clone, Args)]
pub struct ShexmapArgs {
    #[clap(value_parser = clap::value_parser!(InputSpec), help = "Input RDF data: FILE, URI or - for stdin")]
    pub data: Vec<InputSpec>,

    #[arg(
        short = 't',
        long = "data-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "RDF data format",
        default_value_t = DataFormatCli::Turtle
    )]
    pub data_format: DataFormatCli,

    #[arg(
        short = 's',
        long = "schema",
        value_name = "INPUT",
        help = "Input ShEx schema (with %Map:{ %} actions that bind): FILE, URI or - for stdin"
    )]
    pub schema: Option<InputSpec>,

    #[arg(
        short = 'f',
        long = "schema-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Input schema format (ShExC, ShExJ, ...)",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub schema_format: ShExFormatCli,

    #[arg(
        short = 'n',
        long = "node",
        value_name = "NODE",
        help = "Input node to map: <iri>, iri or _:label"
    )]
    pub node: Option<String>,

    #[arg(
        short = 'l',
        long = "shape-label",
        value_name = "LABEL",
        help = "Input shape label (default: the input schema's start)"
    )]
    pub shape: Option<String>,

    #[arg(
        short = 'j',
        long = "bindings",
        value_name = "FILE",
        help = "Read bindings JSON (as written by --bindings-out, shex.js or PyShEx) instead of binding data",
        conflicts_with_all = ["data", "node"]
    )]
    pub bindings: Option<InputSpec>,

    #[arg(
        short = 'b',
        long = "bindings-out",
        value_name = "FILE",
        help = "Write the bindings as JSON here ('-' for the output)"
    )]
    pub bindings_out: Option<String>,

    #[arg(
        short = 'O',
        long = "output-schema",
        value_name = "INPUT",
        help = "Output ShEx schema (with %Map:{ %} actions that place the bindings): FILE, URI or - for stdin"
    )]
    pub output_schema: Option<InputSpec>,

    #[arg(
        long = "output-schema-format",
        ignore_case = true,
        value_name = "FORMAT",
        help = "Output schema format (ShExC, ShExJ, ...)",
        default_value_t = ShExFormatCli::ShExC
    )]
    pub output_schema_format: ShExFormatCli,

    #[arg(
        long = "root",
        value_name = "NODE",
        help = "Output node to build: <iri>, iri or _:label (default: a blank node)"
    )]
    pub root: Option<String>,

    #[arg(
        long = "output-shape",
        value_name = "LABEL",
        help = "Output shape label (default: the output schema's start)"
    )]
    pub output_shape: Option<String>,

    #[arg(
        long = "static",
        value_name = "FILE",
        help = "JSON object of extra variable values (\"<iri>\": \"\\\"literal\\\"\" or a {\"value\": ...} term), as shex.js's staticVars"
    )]
    pub static_vars: Option<PathBuf>,

    #[arg(
        long = "strict",
        help = "Fail when the input or the output can be matched in more than one way",
        default_value_t = false
    )]
    pub strict: bool,

    #[arg(
        long = "no-validate",
        help = "Skip the ShEx validator's check of the input node before binding",
        default_value_t = false
    )]
    pub no_validate: bool,

    #[arg(
        long = "into",
        value_name = "FILE",
        help = "An existing RDF file (in --result-format) to update in place: what the output schema currently holds at --root is replaced, the rest kept; written back there unless --output-file says otherwise"
    )]
    pub into: Option<PathBuf>,

    #[arg(
        long = "provenance",
        value_name = "FILE",
        help = "Write one JSON object per output triple here: the triple, the output constraint's predicate, the input scope and node it came from, the bindings read, and how the object arose"
    )]
    pub provenance: Option<PathBuf>,

    #[arg(
        long = "check",
        help = "Only analyse --schema against --output-schema, without data: every output repetition has a list to iterate, every variable read is bound where it can be read; exit 1 on errors",
        default_value_t = false
    )]
    pub check: bool,

    #[arg(
        long = "base-schema",
        value_name = "IRI",
        help = "Base IRI for relative IRIs in the schemas and shape labels"
    )]
    pub base_schema: Option<String>,

    #[arg(
        long = "base-data",
        value_name = "IRI",
        help = "Base IRI for relative IRIs in the RDF data and nodes"
    )]
    pub base_data: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF reader mode (strict or lax)",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "RDF output format for the materialized graph (Turtle, NTriples, ...)",
        default_value_t = ResultDataFormatCli::Turtle
    )]
    pub result_format: ResultDataFormatCli,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}
