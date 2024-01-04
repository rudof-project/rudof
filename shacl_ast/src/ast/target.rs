use std::fmt::Display;

use prefixmap::IriRef;
use srdf::RDFNode;

#[derive(Debug, Clone)]
pub enum Target {
    TargetNode(RDFNode),
    TargetClass(RDFNode),
    TargetSubjectsOf(IriRef),
    TargetObjectsOf(IriRef),
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::TargetNode(node) => write!(f, "targetNode({node})"),
            Target::TargetClass(node) => write!(f, "targetClass({node})"),
            Target::TargetSubjectsOf(node) => write!(f, "targetSubjectsOf({node})"),
            Target::TargetObjectsOf(node) => write!(f, "targetObjectsOf({node})"),
        }
    }
}
