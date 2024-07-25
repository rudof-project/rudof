use clap::Parser;
use manifest::Manifest;
use oxigraph::{model::Term, store::Store};
use shacl_ast::Schema;
use shacl_validation::{validate::validate, validation_report::report::ValidationReport};
use testsuite_error::TestSuiteError;

mod helper;
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
        default_value = "shacl_testsuite/data-shapes/data-shapes-test-suite/tests/manifest.ttl"
    )]
    manifest_filename: String,
}

struct ShaclTest {
    node: Term,
    manifest_store: Store,
    data_store: Store,
    schema: Schema,
    result: ValidationReport,
    label: Option<String>,
}

impl ShaclTest {
    fn new(
        node: Term,
        manifest_store: Store,
        data_store: Store,
        schema: Schema,
        result: ValidationReport,
        label: Option<String>,
    ) -> Self {
        ShaclTest {
            node,
            manifest_store,
            data_store,
            schema,
            result,
            label,
        }
    }
}

fn main() -> Result<(), TestSuiteError> {
    let cli = Cli::parse(); // we obtain the CLI...

    let manifest = Manifest::load(&cli.manifest_filename)?;

    let mut manifests = Vec::new();
    Manifest::flatten(&manifest, &mut manifests);

    let mut tests = Vec::new();
    for manifest in manifests {
        tests.extend(manifest.collect_tests()?);
    }

    let total = tests.len();
    let mut count = 0;
    for test in tests {
        match validate(&test.data_store, test.schema) {
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
