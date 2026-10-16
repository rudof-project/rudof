use crate::common::TestSuiteError;
use crate::common::manifest::ManifestGraph;
use oxrdf::NamedOrBlankNode;

/// A manifest entry
pub(crate) trait ManifestEntry: Sized {
    fn parse(manifest: &mut ManifestGraph, entry: NamedOrBlankNode) -> Result<Self, TestSuiteError>;
}
