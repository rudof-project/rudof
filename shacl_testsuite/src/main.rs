use std::{fs::File, io::BufReader, path::Path};

use clap::Parser;
use manifest::{GraphManifest, Manifest};
use shacl_validation::{
    shacl_processor::{GraphValidation, ShaclProcessor, ShaclValidationMode},
    store::ShaclDataManager,
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
        default_value_t = ShaclValidationMode::Native,
        value_enum
    )]
    mode: ShaclValidationMode,
}

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
        let validator = GraphValidation::new(
            Path::new(&test.data),
            RDFFormat::Turtle,
            test.base.as_deref(),
            cli.mode,
        )?;
        let file = File::open(test.shapes.as_str())
            .unwrap_or_else(|_| panic!("Unable to open file: {}", test.shapes));
        let reader = BufReader::new(file);
        let schema = ShaclDataManager::load(reader, srdf::RDFFormat::Turtle, test.base.as_deref())?;
        let label = match test.label {
            Some(label) => label,
            None => String::from("Test"),
        };
        match validator.validate(schema) {
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
