use prefixmap::IriRef;
use srdf::RDFNode;

#[derive(Debug, Clone)]
pub enum Target {
    TargetNode(RDFNode),
    TargetClass(RDFNode),
    TargetSubjectsOf(IriRef),
    TargetObjectsOf(IriRef)
}