use std::fmt::Display;

use crate::{SH_TARGET_CLASS, SH_TARGET_NODE};
use prefixmap::IriRef;
use srdf::{RDFNode, SRDFBuilder, RDF_TYPE};

#[derive(Debug, Clone)]
pub enum Target {
    TargetNode(RDFNode),
    TargetClass(RDFNode),
    TargetSubjectsOf(IriRef),
    TargetObjectsOf(IriRef),
    TargetImplicitClass(RDFNode),
}

impl Target {
    // TODO: this is a bit ugly
    pub fn write<RDF>(&self, rdf_node: &RDFNode, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: SRDFBuilder,
    {
        let node: RDF::Subject = rdf_node.clone().try_into().map_err(|_| unreachable!())?;
        match self {
            Self::TargetNode(target_rdf_node) => {
                rdf.add_triple(node, SH_TARGET_NODE.clone(), target_rdf_node.clone())
            }
            Self::TargetClass(target_rdf_node) => {
                rdf.add_triple(node, SH_TARGET_CLASS.clone(), target_rdf_node.clone())
            }
            Self::TargetSubjectsOf(iri_ref) => rdf.add_triple(
                node,
                SH_TARGET_CLASS.clone(),
                iri_ref.get_iri().unwrap().clone(),
            ),
            Self::TargetObjectsOf(iri_ref) => rdf.add_triple(
                node,
                SH_TARGET_CLASS.clone(),
                iri_ref.get_iri().unwrap().clone(),
            ),
            // TODO: check if this is fine
            Self::TargetImplicitClass(target_rdf_node) => {
                rdf.add_triple(node, RDF_TYPE.clone(), target_rdf_node.clone())
            }
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
            Target::TargetImplicitClass(node) => write!(f, "targetImplicitClass({node})"),
        }
    }
}
