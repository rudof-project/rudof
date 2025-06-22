use std::fmt::Display;

use crate::shacl_vocab::{
    sh_target_class, sh_target_node, sh_target_objects_of, sh_target_subjects_of,
};
use prefixmap::IriRef;
use srdf::{rdf_type, rdfs_class, BuildRDF, RDFNode};

#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    TargetNode(RDFNode), // TODO: Shacl12: Extend to Node Expressions
    TargetClass(RDFNode),
    TargetSubjectsOf(IriRef),
    TargetObjectsOf(IriRef),
    TargetImplicitClass(RDFNode),
}

impl Target {
    pub fn target_node(node: RDFNode) -> Self {
        Target::TargetNode(node)
    }
    pub fn target_class(node: RDFNode) -> Self {
        Target::TargetClass(node)
    }
    pub fn target_subjects_of(iri_ref: IriRef) -> Self {
        Target::TargetSubjectsOf(iri_ref)
    }
    pub fn target_objects_of(iri_ref: IriRef) -> Self {
        Target::TargetObjectsOf(iri_ref)
    }
    pub fn target_implicit_class(node: RDFNode) -> Self {
        Target::TargetImplicitClass(node)
    }
    pub fn write<RDF>(&self, rdf_node: &RDFNode, rdf: &mut RDF) -> Result<(), RDF::Err>
    where
        RDF: BuildRDF,
    {
        let node: RDF::Subject = rdf_node.clone().try_into().map_err(|_| unreachable!())?;
        match self {
            Self::TargetNode(target_rdf_node) => {
                rdf.add_triple(node, sh_target_node().clone(), target_rdf_node.clone())
            }
            Self::TargetClass(node_class) => {
                rdf.add_triple(node, sh_target_class().clone(), node_class.clone())
            }
            Self::TargetSubjectsOf(iri_ref) => rdf.add_triple(
                node,
                sh_target_subjects_of().clone(),
                iri_ref.get_iri().unwrap().clone(),
            ),
            Self::TargetObjectsOf(iri_ref) => rdf.add_triple(
                node,
                sh_target_objects_of().clone(),
                iri_ref.get_iri().unwrap().clone(),
            ),
            // TODO: we have to add rdfs:Class
            Self::TargetImplicitClass(_class) => {
                // TODO: Review this code and in SHACL 1.2, add sh_shape_class ?
                rdf.add_triple(node, rdf_type().clone(), rdfs_class().clone())
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
