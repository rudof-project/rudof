use clap::Parser;
use manifest::{GraphManifest, Manifest};
use shacl_validation::{
    validate::{GraphValidator, Validator},
    validation_report::report::ValidationReport,
};
use srdf::{SRDFBasic, SRDF};
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
struct ShaclTest<S: SRDF + SRDFBasic> {
    data: String,
    shapes: String,
    base: Option<String>,
    result: ValidationReport<S>,
    label: Option<String>,
}

impl<S: SRDF + SRDFBasic> ShaclTest<S> {
    fn new(
        data: String,
        shapes: String,
        base: Option<String>,
        result: ValidationReport<S>,
        label: Option<String>,
    ) -> Self {
        ShaclTest {
            data,
            shapes,
            base,
            result,
            label,
        }
    }
}

fn main() -> Result<(), TestSuiteError> {
    let cli = Cli::parse(); // we obtain the CLI...

    let manifest = GraphManifest::load(&cli.manifest_filename)?;

    let mut manifests = Vec::new();
    Manifest::flatten(manifest, &mut manifests);

    let mut tests = Vec::new();
    for manifest in manifests {
        tests.extend(manifest.collect_tests()?);
    }

    let total = tests.len();
    let mut count = 0;
    for test in tests {
        let validator =
            GraphValidator::new(&test.data, srdf::RDFFormat::NTriples, test.base.as_deref());
        match validator.validate(&test.shapes, srdf::RDFFormat::Turtle) {
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
