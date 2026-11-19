use crate::TestSuiteError;
use crate::manifest::ManifestGraph;
use oxrdf::NamedOrBlankNode;

/// A manifest entry
pub trait ManifestEntry: Sized {
    fn parse(manifest: &mut ManifestGraph, entry: NamedOrBlankNode) -> Result<Self, TestSuiteError>;
}
