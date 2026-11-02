use crate::cli::parser::CommonArgsNoBackend;
use crate::cli::wrappers::{ComparisonModeCli, DataReaderModeCli, ResultComparisonFormatCli, SubgraphModeCli};
use clap::{Args, ValueEnum};
use rudof_lib::formats::InputSpec;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

/// Arguments for the `compare` command
#[derive(Debug, Clone, Args)]
pub struct CompareArgs {
    #[arg(
        long = "mode",
        value_name = "MODE",
        ignore_case = true,
        help = "What to compare: two schemas, or two RDF graphs [default: schema with --schema1 and --schema2, \
                data with --data1 and --data2]",
        value_enum
    )]
    pub mode: Option<CompareModeCli>,

    #[arg(long = "mode1",
        value_name = "MODE",
        ignore_case = true,
        help = "Input mode first schema (schema mode)",
        default_value_t = ComparisonModeCli::ShEx
    )]
    pub input_mode1: ComparisonModeCli,

    #[arg(
        long = "mode2",
        value_name = "MODE",
        ignore_case = true,
        help = "Input mode second schema (schema mode)",
        default_value_t = ComparisonModeCli::ShEx
    )]
    pub input_mode2: ComparisonModeCli,

    #[arg(
        long = "schema1",
        value_name = "INPUT",
        help = "First schema (URI, file or - for stdin)"
    )]
    pub schema1: Option<InputSpec>,

    #[arg(
        long = "schema2",
        value_name = "INPUT",
        help = "Second schema (URI, file or - for stdin)"
    )]
    pub schema2: Option<InputSpec>,

    #[arg(
        long = "data1",
        value_name = "INPUT",
        help = "First RDF graph (URI, file or - for stdin). If omitted with --data2, the loaded data (in the shell)"
    )]
    pub data1: Option<InputSpec>,

    #[arg(
        long = "data2",
        value_name = "INPUT",
        help = "Second RDF graph (URI, file or - for stdin)"
    )]
    pub data2: Option<InputSpec>,

    #[arg(
        long = "format1",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Format of the first input [default: shexc in schema mode, turtle in data mode]",
        value_enum
    )]
    pub format1: Option<CompareFormatCli>,

    #[arg(
        long = "format2",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Format of the second input [default: shexc in schema mode, turtle in data mode]",
        value_enum
    )]
    pub format2: Option<CompareFormatCli>,

    #[arg(long = "base1", value_name = "IRI", help = "Base IRI of the first input")]
    pub base1: Option<String>,

    #[arg(long = "base2", value_name = "IRI", help = "Base IRI of the second input")]
    pub base2: Option<String>,

    #[arg(
        long = "subgraph-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "Data mode: how blank nodes may be mapped when checking containment: one-to-one onto blank \
                nodes (injective), onto blank nodes (homomorphic), or onto any term, i.e. simple entailment (entailment)",
        default_value_t = SubgraphModeCli::Injective
    )]
    pub subgraph_mode: SubgraphModeCli,

    #[arg(
        long = "show-diff",
        help = "Data mode: also show the triples of each graph that the other one does not have"
    )]
    pub show_diff: bool,

    #[arg(
        long = "exact-search-limit",
        value_name = "STEPS",
        help = "Data mode, with --show-diff: maximum steps of the search for the best blank node mapping; past it \
                the diff is approximate [default: 100000]"
    )]
    pub exact_search_limit: Option<usize>,

    #[arg(
        short = 'r',
        long = "result-format",
        value_name = "FORMAT",
        ignore_case = true,
        help = "Result format",
        default_value_t = ResultComparisonFormatCli::Internal
    )]
    pub result_format: ResultComparisonFormatCli,

    #[arg(short = 't', long = "target-folder", value_name = "FOLDER", help = "Target folder")]
    pub target_folder: Option<PathBuf>,

    #[arg(
        long = "shape1",
        value_name = "LABEL",
        help = "shape1 (default = START, schema mode)"
    )]
    pub shape1: Option<String>,

    #[arg(
        long = "shape2",
        value_name = "LABEL",
        help = "shape2 (default = START, schema mode)"
    )]
    pub shape2: Option<String>,

    #[arg(
        long = "reader-mode",
        value_name = "MODE",
        ignore_case = true,
        help = "RDF Reader mode",
        default_value_t = DataReaderModeCli::Strict,
        value_enum
    )]
    pub reader_mode: DataReaderModeCli,

    #[arg(long = "show-time", help = "Show processing time")]
    pub show_time: Option<bool>,

    #[command(flatten)]
    pub common: CommonArgsNoBackend,
}

/// What the `compare` command compares
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum CompareModeCli {
    /// Two schemas (ShEx, SHACL, ...), shape by shape
    Schema,
    /// Two RDF graphs, up to blank node renaming
    Data,
}

/// Input formats of the `compare` command: the schema formats and the RDF
/// formats. Each mode accepts only its own.
#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
#[clap(rename_all = "lower")]
pub enum CompareFormatCli {
    ShExC,
    ShExJ,
    Turtle,
    NTriples,
    RdfXml,
    TriG,
    N3,
    NQuads,
    JsonLd,
}

macro_rules! impl_display_via_value_enum {
    ($($t:ty),*) => {$(
        impl Display for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                let value = self.to_possible_value().expect("no skipped variants");
                write!(f, "{}", value.get_name())
            }
        }
    )*};
}

impl_display_via_value_enum!(CompareModeCli, CompareFormatCli);
