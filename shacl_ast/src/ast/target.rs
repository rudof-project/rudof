use std::fmt::Display;

use crate::{SH_TARGET_CLASS, SH_TARGET_NODE};
use prefixmap::IriRef;
use srdf::{RDFNode, SRDFBuilder};

#[derive(Debug, Clone)]
pub enum Target {
    TargetNode(RDFNode),
    TargetClass(RDFNode),
    TargetSubjectsOf(IriRef),
    TargetObjectsOf(IriRef),
}

impl Target {
    // TODO: this is a bit ugly
    pub fn write<RDF>(&self, rdf_node: &RDFNode, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        match self {
            Self::TargetNode(target_rdf_node) => rdf.add_triple(
                &RDF::object_as_subject(rdf_node).unwrap(),
                &SH_TARGET_NODE.clone().into(),
                &RDF::object_as_term(target_rdf_node),
            ),
            Self::TargetClass(target_rdf_node) => rdf.add_triple(
                &RDF::object_as_subject(rdf_node).unwrap(),
                &SH_TARGET_CLASS.clone().into(),
                &RDF::object_as_term(target_rdf_node),
            ),
            Self::TargetSubjectsOf(iri_ref) => rdf.add_triple(
                &RDF::object_as_subject(rdf_node).unwrap(),
                &SH_TARGET_CLASS.clone().into(),
                &RDF::iri_s2term(&iri_ref.get_iri().unwrap()),
            ),
            Self::TargetObjectsOf(iri_ref) => rdf.add_triple(
                &RDF::object_as_subject(rdf_node).unwrap(),
                &SH_TARGET_CLASS.clone().into(),
                &RDF::iri_s2term(&iri_ref.get_iri().unwrap()),
            ),
        }
    }
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
