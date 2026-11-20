use crate::TestSuiteError;
use crate::manifest::ManifestGraph;
use oxrdf::{NamedNode, NamedOrBlankNode, Term};
use rudof_rdf::rdf_core::vocabs::{RdfVocab, ShaclTestVocab, TestManifestVocab};
use std::path::{Path, PathBuf};

/// An approved `sht:Validate` entry
pub struct ValidateEntry {
    /// The entry's IRI
    pub entry: String,
    /// The test file it is defined in
    pub path: PathBuf,
}

/// Walks a test suite's `mf:include` tree from `tests/manifest.ttl`
pub struct ManifestWalker {
    tests_root: PathBuf,
}

impl ManifestWalker {
    pub fn new(tests_root: impl Into<PathBuf>) -> Self {
        Self {
            tests_root: tests_root
                .into()
                .join("tests"),
        }
    }

    pub fn validate_entries(&self) -> Result<Vec<ValidateEntry>, TestSuiteError> {
        let mut out = Vec::new();
        self.walk(Path::new("manifest.ttl"), &mut out)?;
        Ok(out)
    }

    fn walk(&self, rel_path: &Path, out: &mut Vec<ValidateEntry>) -> Result<(), TestSuiteError> {
        let disk_path = self.tests_root.join(rel_path);
        let base = format!("urn:x-shacl-test:/{}", rel_path.display());
        let graph = ManifestGraph::new_with_base(&disk_path, &base)?;
        let subject = NamedOrBlankNode::NamedNode(NamedNode::new_unchecked(base));

        let mf_include: NamedNode = TestManifestVocab::mf_include().into();
        for include in graph.values(&subject, &mf_include)? {
            if let Some(rel) = as_urn_path(&include) {
                self.walk(Path::new(rel), out)?;
            }
        }

        let rdf_type: NamedNode = RdfVocab::rdf_type().into();
        let mf_status: NamedNode = TestManifestVocab::mf_status().into();
        let sht_validate = Term::NamedNode(ShaclTestVocab::sht_validate().into());
        let sht_approved = Term::NamedNode(ShaclTestVocab::sht_approved().into());

        for entry in graph.entries() {
            let entry_subject: NamedOrBlankNode = entry.clone().try_into()?;

            let is_validate = graph.value(&entry_subject, &rdf_type)?.as_ref() == Some(&sht_validate);
            let is_approved = graph.value(&entry_subject, &mf_status)?.as_ref() == Some(&sht_approved);

            if is_validate
                && is_approved
                && let Term::NamedNode(iri) = entry
            {
                out.push(ValidateEntry {
                    entry: iri.as_str().to_string(),
                    path: disk_path.clone(),
                });
            }
        }

        Ok(())
    }
}

/// Strips the `urn:x-shacl-test:/` prefix off a resolved `mf:include` IRI
fn as_urn_path(term: &Term) -> Option<&str> {
    match term {
        Term::NamedNode(iri) => iri.as_str().strip_prefix("urn:x-shacl-test:/"),
        _ => None,
    }
}
