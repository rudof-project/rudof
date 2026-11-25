//! Runs every approved entry of the vendored W3C SHACL test suites.
//!
//! You can run a given test by name:
//! ```sh
//! cargo test -p shacl --test shacl_testsuite -- disjoint-001
//! ```
//!
//! Also you can use the `--ignored` flag to list the ignored tests by the `ignored.toml` file.
//! To refresh this file after fixing or regressing a batch of tests, run:
//!
//! ```sh
//! cargo test -p shacl --test shacl_testsuite -- --bless
//! ```

#[cfg(not(target_family = "wasm"))]
mod run {
    use libtest_mimic::{Arguments, Failed, Trial};
    use shacl_testsuite::TestSuiteError;
    use shacl_testsuite::ignored::{IgnoredEntry, IgnoredList};
    use shacl_testsuite::manifest::{ManifestTestEntry, ManifestWalker};
    use shacl_testsuite::runner::{TestOutcome, node_expr_outcome, run_outcome};
    use std::path::{Path, PathBuf};

    const IGNORED_TOML: &str = "tests/ignored.toml";

    #[derive(Clone, Copy)]
    enum Kind {
        Validate,
        NodeExpr,
    }

    struct Entry {
        /// `<suite>::<test IRI>`, e.g. `data-shapes::urn:x-shacl-test:/core/node/disjoint-001`
        name: String,
        kind: Kind,
        path: PathBuf,
    }

    fn suite_entries(suite: &str, root: &str) -> Result<Vec<Entry>, TestSuiteError> {
        let walker = ManifestWalker::new(root);
        let mut out = Vec::new();

        let as_entry = |suite: String, kind: Kind| {
            move |e: ManifestTestEntry| Entry {
                name: format!("{suite}::{}", e.entry),
                kind,
                path: e.path,
            }
        };

        out.extend(
            walker
                .validate_entries()?
                .into_iter()
                .map(as_entry(suite.to_owned(), Kind::Validate)),
        );
        out.extend(
            walker
                .node_expr_entries()?
                .into_iter()
                .map(as_entry(suite.to_owned(), Kind::NodeExpr)),
        );

        Ok(out)
    }

    fn all_entries() -> Result<Vec<Entry>, TestSuiteError> {
        let mut out = suite_entries("data-shapes", "tests/data-shapes/data-shapes-test-suite")?;
        out.extend(suite_entries("shacl12", "tests/data-shapes/shacl12-test-suite")?);
        Ok(out)
    }

    fn run_entry(entry: &Entry) -> TestOutcome {
        match entry.kind {
            Kind::Validate => run_outcome(&entry.path),
            Kind::NodeExpr => node_expr_outcome(&entry.path),
        }
    }

    /// Re-runs every entry and rewrites `tests/ignored.toml`
    fn bless(entries: &[Entry]) -> Result<(), TestSuiteError> {
        let path = Path::new(IGNORED_TOML);
        let previous = IgnoredList::load(path)?;

        let mut ignored = Vec::new();
        let (mut added, mut removed) = (0_usize, 0_usize);

        for entry in entries {
            match run_entry(entry) {
                TestOutcome::Passed => {
                    if previous.is_ignored(&entry.name) {
                        removed += 1;
                    }
                },
                TestOutcome::Failed(_) => {
                    if !previous.is_ignored(&entry.name) {
                        added += 1;
                    }
                    ignored.push(IgnoredEntry {
                        test: entry.name.clone(),
                        reason: previous.reason_for(&entry.name).map(str::to_owned),
                        issue: None,
                    });
                },
            }
        }

        IgnoredList { ignored }.save(path)?;
        eprintln!("tests/ignored.toml updated: {added} added, {removed} removed");
        Ok(())
    }

    pub fn main() -> Result<(), TestSuiteError> {
        let mut raw_args: Vec<String> = std::env::args().collect();
        let bless_requested = raw_args.iter().any(|a| a == "--bless");
        raw_args.retain(|a| a != "--bless");

        let entries = all_entries()?;

        if bless_requested {
            return bless(&entries);
        }

        let args = Arguments::from_iter(raw_args.into_iter().skip(1));
        let ignored = IgnoredList::load(Path::new(IGNORED_TOML))?;

        let trials = entries
            .into_iter()
            .map(|entry| {
                let is_known = ignored.is_ignored(&entry.name);
                Trial::test(entry.name.clone(), move || match (is_known, run_entry(&entry)) {
                    (true, TestOutcome::Failed(_)) => Ok(()),
                    (true, TestOutcome::Passed) => Err(Failed::from(
                        "listed in tests/ignored.toml as failing but it now passes; remove the entry",
                    )),
                    (false, TestOutcome::Failed(message)) => Err(Failed::from(message)),
                    (false, TestOutcome::Passed) => Ok(()),
                })
            })
            .collect();

        libtest_mimic::run(&args, trials).exit();
    }
}

fn main() {
    #[cfg(not(target_family = "wasm"))]
    run::main().unwrap();
}
