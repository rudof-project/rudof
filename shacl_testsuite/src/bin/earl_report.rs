//! Walks a SHACL W3C test suite and builds an EARL report
//!
//! ```sh
//! cargo run -p shacl_testsuite --bin earl_report -- \
//!   --suite shacl/tests/data-shapes/data-shapes-test-suite \
//!   --out shacl_testsuite/reports/rudof-shacl-earl.ttl
//! ```

#[cfg(not(target_family = "wasm"))]
mod run {
    use clap::Parser;
    use rudof_earl::model::{Assertion, TestResult};
    use rudof_earl::report::EarlReport;
    use rudof_earl::{Release, Software};
    use rudof_iri::IriS;
    use rudof_rdf::rdf_core::BuildRDF;
    use rudof_rdf::rdf_core::RDFFormat;
    use rudof_rdf::rdf_core::term::Object;
    use rudof_rdf::rdf_core::term::literal::ConcreteLiteral;
    use rudof_rdf::rdf_impl::OxigraphInMemory;
    use shacl_testsuite::manifest::ManifestWalker;
    use shacl_testsuite::runner::{TestOutcome, run_outcome};
    use std::fs;
    use std::path::PathBuf;

    // `earl:subject`
    const TOOL: &str = "https://github.com/rudof-project/rudof";
    // `earl:assertedBy`
    const ORG: &str = "https://github.com/rudof-project";

    #[derive(Parser)]
    struct Args {
        /// Root of the test suite
        #[arg(long)]
        suite: PathBuf,

        /// Where to write the Turtle EARL report
        #[arg(long)]
        out: PathBuf,
    }

    pub fn main() -> anyhow::Result<()> {
        let args = Args::parse();

        let entries = ManifestWalker::new(&args.suite).validate_entries()?;

        let tool = IriS::new_unchecked(TOOL);
        let org = IriS::new_unchecked(ORG);

        let mut report = EarlReport::new().with_software_item(
            Into::<Software>::into(tool.clone())
                .with_name("rudof")
                .with_developer(Object::iri(org.clone()))
                .with_homepage(IriS::new_unchecked("https://rudof-project.github.io/rudof"))
                .with_programming_language("Rust")
                .with_release(Release::new(env!("CARGO_PKG_VERSION"))),
        );

        let (mut passed, mut failed) = (0_usize, 0_usize);

        for entry in &entries {
            let result = match run_outcome(&entry.path) {
                TestOutcome::Passed => {
                    passed += 1;
                    TestResult::passed()
                },
                TestOutcome::Failed(message) => {
                    failed += 1;
                    TestResult::failed().with_info(ConcreteLiteral::str(&message))
                },
            };

            report = report.with_assertion(
                Assertion::new(
                    Object::iri(tool.clone()),
                    Object::iri(IriS::new_unchecked(&entry.entry)),
                    result,
                )
                .with_asserted_by(Object::iri(org.clone())),
            );
        }

        let mut graph = OxigraphInMemory::new();
        report.to_rdf(&mut graph)?;

        let mut out = Vec::new();
        graph.serialize(&RDFFormat::Turtle, &mut out)?;
        fs::write(&args.out, out)?;

        eprintln!(
            "{} entries: {passed} passed, {failed} failed -> {}",
            entries.len(),
            args.out.display()
        );

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    #[cfg(not(target_family = "wasm"))]
    return run::main();

    #[cfg(target_family = "wasm")]
    Ok(())
}
