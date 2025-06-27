use std::fmt::Display;

use crate::shacl_vocab::{
    sh_target_class, sh_target_node, sh_target_objects_of, sh_target_subjects_of,
};
use prefixmap::IriRef;
use srdf::{rdf_type, rdfs_class, BuildRDF, RDFNode, Rdf};

/// Represents target declarations
#[derive(Debug)]
pub enum Target<S: Rdf>
 where S::Term: Clone {
    TargetNode(RDFNode), // TODO: Shacl12: Extend to Node Expressions
    TargetClass(RDFNode),
    TargetSubjectsOf(IriRef),
    TargetObjectsOf(IriRef),
    TargetImplicitClass(RDFNode),

    // The following target declaration are not well formed but we keep them to generate violation errors for them
    WrongTargetNode(S::Term),
    WrongTargetClass(S::Term),
    WrongSubjectsOf(S::Term),
    WrongObjectsOf(S::Term),
    WrongImplicitClass(S::Term)

}

impl<S:Rdf> Target<S> {
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
            Target::TargetNode(target_rdf_node) => {
                        rdf.add_triple(node, sh_target_node().clone(), target_rdf_node.clone())
                    }
            Target::TargetClass(node_class) => {
                        rdf.add_triple(node, sh_target_class().clone(), node_class.clone())
                    }
            Target::TargetSubjectsOf(iri_ref) => rdf.add_triple(
                        node,
                        sh_target_subjects_of().clone(),
                        iri_ref.get_iri().unwrap().clone(),
                    ),
            Target::TargetObjectsOf(iri_ref) => rdf.add_triple(
                        node,
                        sh_target_objects_of().clone(),
                        iri_ref.get_iri().unwrap().clone(),
                    ),
            Target::TargetImplicitClass(_class) => {
                        // TODO: Review this code and in SHACL 1.2, add sh_shape_class ?
                        rdf.add_triple(node, rdf_type().clone(), rdfs_class().clone())
                    }
            Target::WrongTargetNode(_) => todo!(),
            Target::WrongTargetClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        }
    }
}

impl<S:Rdf> Display for Target<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::TargetNode(node) => write!(f, "targetNode({node})"),
            Target::TargetClass(node) => write!(f, "targetClass({node})"),
            Target::TargetSubjectsOf(node) => write!(f, "targetSubjectsOf({node})"),
            Target::TargetObjectsOf(node) => write!(f, "targetObjectsOf({node})"),
            Target::TargetImplicitClass(node) => write!(f, "targetImplicitClass({node})"),
            Target::WrongTargetNode(node) => write!(f, "targetNode({node})"),
            Target::WrongTargetClass(node) => write!(f, "targetClass({node})"),
            Target::WrongSubjectsOf(node) => write!(f, "targetSubjectsOf({node})"),
            Target::WrongObjectsOf(node) => write!(f, "targetObjectsOf({node})"),
            Target::WrongImplicitClass(node) => write!(f, "targetImplicitClass({node})"),
        }
    }
}

impl <RDF: Rdf> Clone for Target<RDF> {
    fn clone(&self) -> Self {
        match self {
            Self::TargetNode(arg0) => Self::TargetNode(arg0.clone()),
            Self::TargetClass(arg0) => Self::TargetClass(arg0.clone()),
            Self::TargetSubjectsOf(arg0) => Self::TargetSubjectsOf(arg0.clone()),
            Self::TargetObjectsOf(arg0) => Self::TargetObjectsOf(arg0.clone()),
            Self::TargetImplicitClass(arg0) => Self::TargetImplicitClass(arg0.clone()),
            Self::WrongTargetNode(arg0) => Self::WrongTargetNode(arg0.clone()),
            Self::WrongTargetClass(arg0) => Self::WrongTargetClass(arg0.clone()),
            Self::WrongSubjectsOf(arg0) => Self::WrongSubjectsOf(arg0.clone()),
            Self::WrongObjectsOf(arg0) => Self::WrongObjectsOf(arg0.clone()),
            Self::WrongImplicitClass(arg0) => Self::WrongImplicitClass(arg0.clone()),
        }
    }
}

impl<RDF:Rdf> PartialEq for Target<RDF> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::TargetNode(l0), Self::TargetNode(r0)) => l0 == r0,
            (Self::TargetClass(l0), Self::TargetClass(r0)) => l0 == r0,
            (Self::TargetSubjectsOf(l0), Self::TargetSubjectsOf(r0)) => l0 == r0,
            (Self::TargetObjectsOf(l0), Self::TargetObjectsOf(r0)) => l0 == r0,
            (Self::TargetImplicitClass(l0), Self::TargetImplicitClass(r0)) => l0 == r0,
            (Self::WrongTargetNode(l0), Self::WrongTargetNode(r0)) => l0 == r0,
            (Self::WrongTargetClass(l0), Self::WrongTargetClass(r0)) => l0 == r0,
            (Self::WrongSubjectsOf(l0), Self::WrongSubjectsOf(r0)) => l0 == r0,
            (Self::WrongObjectsOf(l0), Self::WrongObjectsOf(r0)) => l0 == r0,
            (Self::WrongImplicitClass(l0), Self::WrongImplicitClass(r0)) => l0 == r0,
            _ => false,
        }
    }
}