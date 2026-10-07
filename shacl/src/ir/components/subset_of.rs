use rudof_rdf::rdf_core::SHACLPath;
use std::fmt::{Display, Formatter};

/// sh:subsetOf specifies the condition that all value nodes must also be
/// reachable from the focus node via the SHACL property path that is specified
/// using sh:subsetOf.
///
/// https://www.w3.org/TR/shacl12-core/#SubsetOfConstraintComponent
#[derive(Debug, Clone)]
pub struct SubsetOf {
    path: SHACLPath,
}

impl SubsetOf {
    pub fn new(path: SHACLPath) -> Self {
        SubsetOf { path }
    }

    pub fn path(&self) -> &SHACLPath {
        &self.path
    }
}

impl Display for SubsetOf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubsetOf: {}", self.path())
    }
}
