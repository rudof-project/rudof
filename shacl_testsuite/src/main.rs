use std::path::Path;

use clap::Parser;
use manifest::{GraphManifest, Manifest};
use shacl_validation::{
    validate::{GraphValidator, Mode, Validator},
    validation_report::report::ValidationReport,
};
use srdf::{RDFFormat, SRDFBasic, SRDF};
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
        short = 'f',
        long = "file",
        value_name = "Manifest FILE (.ttl)",
        default_value = "shacl_testsuite/data-shapes/data-shapes-test-suite/tests/manifest.ttl"
    )]
    manifest_filename: String,
    /// Execution mode
    #[arg(
        short = 'm',
        long = "mode",
        value_name = "Execution mode",
        default_value_t = Mode::Default,
        value_enum
    )]
    mode: Mode,
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

#[allow(clippy::result_large_err)]
fn main() -> Result<(), TestSuiteError> {
    let cli = Cli::parse(); // we obtain the CLI...
    let path = Path::new(&cli.manifest_filename);
    let manifest = GraphManifest::load(path)?;
    let mut manifests = Vec::new();
    let mut tests = Vec::new();

    Manifest::flatten(manifest, &mut manifests);

    for manifest in manifests {
        tests.extend(manifest.collect_tests()?);
    }

    let total = tests.len();
    let mut count = 0;
    for test in tests {
        let validator = GraphValidator::new(
            Path::new(&test.data),
            RDFFormat::NTriples,
            test.base.as_deref(),
            cli.mode,
        )?;
        let label = match test.label {
            Some(label) => label,
            None => String::from("Test"),
        };
        match validator.validate(Path::new(&test.shapes), srdf::RDFFormat::Turtle) {
            Ok(actual) => {
                if actual == test.result {
                    println!("{} succeeded", label);
                    count += 1;
                } else {
                    println!("{} failed", label);
                    println!("Actual: {}", actual);
                    println!("Expected: {}", test.result);
                }
            }
            Err(error) => {
                eprintln!("{} - {}", label, error)
            }
        };
    }

    println!("{}/{}", count, total);

    Ok(())
}
