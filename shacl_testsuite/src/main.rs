use clap::Parser;
use manifest::Manifest;
use shacl_ast::Schema;
use shacl_validation::validate::validate;
use shacl_validation::validation_report::report::ValidationReport;
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

// TODO: The following line is to make clippy happy...should be removed, it complains that node and manifest_store are not used
#[allow(dead_code)]
struct ShaclTest<S, T> {
    node: T,
    manifest_store: S,
    data_store: S,
    schema: Schema,
    result: ValidationReport,
    label: Option<String>,
}

impl<S, T> ShaclTest<S, T> {
    fn new(
        node: T,
        manifest_store: S,
        data_store: S,
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
                let label = match test.label {
                    Some(label) => label,
                    None => String::from("Test"),
                };
                if actual == test.result {
                    println!("{} succeeded", label);
                    count += 1;
                } else {
                    println!("{} failed", label);
                }
            }
            Err(_) => todo!(),
        };
    }

    println!("{}/{}", count, total);

    Ok(())
}
