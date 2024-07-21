use clap::Parser;
use manifest::Manifest;
use oxrdf::Term;
use shacl_ast::Schema;
use shacl_validation::{validate::validate, validation_report::report::ValidationReport};
use srdf::SRDFGraph;
use testsuite_error::TestSuiteError;

mod manifest;
mod manifest_error;
mod testsuite_error;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Name of Manifest file
    #[arg(
        short = 'm',
        long = "manifest",
        value_name = "Manifest FILE (.ttl)",
        default_value = "/shacl_testsuite/data-shapes/data-shapes-test-suite/tests/core.ttl"
    )]
    manifest_filename: String,
}

struct ShaclTest {
    node: Term,
    graph: SRDFGraph,
    schema: Schema,
    result: ValidationReport,
    data_graph: SRDFGraph,
    label: Option<String>,
}

impl ShaclTest {
    fn new(
        node: Term,
        graph: SRDFGraph,
        schema: Schema,
        result: ValidationReport,
        data_graph: SRDFGraph,
        label: Option<String>,
    ) -> Self {
        ShaclTest {
            node,
            graph,
            schema,
            result,
            data_graph,
            label,
        }
    }
}

fn main() -> Result<(), TestSuiteError> {
    let cli = Cli::parse(); // we obtain the CLI...

    let manifest = match Manifest::load(&cli.manifest_filename) {
        Some(manifest) => manifest,
        None => todo!(),
    };

    let mut manifests = Vec::new();
    Manifest::flatten(&manifest, &mut manifests);

    let mut tests = Vec::new();
    for manifest in manifests {
        tests.extend(manifest.collect_tests());
    }

    let total = tests.len();
    let mut count = 0;
    for test in tests {
        match validate(&test.data_graph, test.schema) {
            Ok(actual) => {
                if actual == test.result {
                    count += 1;
                }
            }
            Err(_) => todo!(),
        };
    }

    println!("{}/{}", count, total);

    Ok(())
}
