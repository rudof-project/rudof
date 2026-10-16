use oxrdf::NamedOrBlankNode;
use crate::common::manifest::ManifestGraph;
use crate::common::TestSuiteError;

/// A manifest entry
pub(crate) trait ManifestEntry: Sized {
    fn parse(manifest: &mut ManifestGraph, entry: NamedOrBlankNode) -> Result<Self, TestSuiteError>;
}