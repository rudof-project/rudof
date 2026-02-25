use std::fmt::Display;

use prefixmap::IriRef;
use rudof_rdf::rdf_core::vocabs::{RdfVocab, RdfsVocab, ShaclVocab};
use rudof_rdf::rdf_core::{
    BuildRDF, Rdf,
    term::Object,
};

/// Represents target declarations
#[derive(Debug)]
pub enum Target<S: Rdf>
where
    S::Term: Clone,
{
    Node(Object), // TODO - Replace with NodeExpr
    Class(Object),
    SubjectsOf(IriRef),
    ObjectsOf(IriRef),
    ImplicitClass(Object),

    // The following target declaration are not well formed but we keep them to generate violation errors for them
    WrongNode(S::Term),
    WrongClass(S::Term),
    WrongSubjectsOf(S::Term),
    WrongObjectsOf(S::Term),
    WrongImplicitClass(S::Term),
}

impl<RDF: Rdf> Target<RDF> {
    pub fn target_node(node: Object) -> Self {
        // TODO - Replace with NodeExpr
        Target::Node(node)
    }
    pub fn target_class(node: Object) -> Self {
        Target::Class(node)
    }
    pub fn target_subjects_of(iri_ref: IriRef) -> Self {
        Target::SubjectsOf(iri_ref)
    }
    pub fn target_objects_of(iri_ref: IriRef) -> Self {
        Target::ObjectsOf(iri_ref)
    }
    pub fn target_implicit_class(node: Object) -> Self {
        Target::ImplicitClass(node)
    }
    pub fn write<B: BuildRDF>(&self, rdf_node: &Object, rdf: &mut B) -> Result<(), B::Err> {
        let node: B::Subject = rdf_node.clone().try_into().map_err(|_| unreachable!())?;
        match self {
            Target::Node(target_rdf_node) => {
                rdf.add_triple(node, ShaclVocab::sh_target_node().clone(), target_rdf_node.clone())
            },
            Target::Class(node_class) => {
                rdf.add_triple(node, ShaclVocab::sh_target_class().clone(), node_class.clone())
            },
            Target::SubjectsOf(iri_ref) => rdf.add_triple(
                node,
                ShaclVocab::sh_target_subjects_of().clone(),
                iri_ref.get_iri().unwrap().clone(),
            ),
            Target::ObjectsOf(iri_ref) => rdf.add_triple(
                node,
                ShaclVocab::sh_target_objects_of().clone(),
                iri_ref.get_iri().unwrap().clone(),
            ),
            Target::ImplicitClass(_class) => {
                // TODO: Review this code and in SHACL 1.2, add sh_shape_class ?
                rdf.add_triple(node, RdfVocab::rdf_type().clone(), RdfsVocab::rdfs_class().clone())
            },
            Target::WrongNode(_) => todo!(),
            Target::WrongClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        }
    }
}

impl<S: Rdf> Display for Target<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Node(node) => write!(f, "targetNode({node})"),
            Target::Class(node) => write!(f, "targetClass({node})"),
            Target::SubjectsOf(node) => write!(f, "targetSubjectsOf({node})"),
            Target::ObjectsOf(node) => write!(f, "targetObjectsOf({node})"),
            Target::ImplicitClass(node) => write!(f, "targetImplicitClass({node})"),
            Target::WrongNode(node) => write!(f, "targetNode({node})"),
            Target::WrongClass(node) => write!(f, "targetClass({node})"),
            Target::WrongSubjectsOf(node) => write!(f, "targetSubjectsOf({node})"),
            Target::WrongObjectsOf(node) => write!(f, "targetObjectsOf({node})"),
            Target::WrongImplicitClass(node) => write!(f, "targetImplicitClass({node})"),
        }
    }
}

impl<RDF: Rdf> Clone for Target<RDF> {
    fn clone(&self) -> Self {
        match self {
            Self::Node(arg0) => Self::Node(arg0.clone()),
            Self::Class(arg0) => Self::Class(arg0.clone()),
            Self::SubjectsOf(arg0) => Self::SubjectsOf(arg0.clone()),
            Self::ObjectsOf(arg0) => Self::ObjectsOf(arg0.clone()),
            Self::ImplicitClass(arg0) => Self::ImplicitClass(arg0.clone()),
            Self::WrongNode(arg0) => Self::WrongNode(arg0.clone()),
            Self::WrongClass(arg0) => Self::WrongClass(arg0.clone()),
            Self::WrongSubjectsOf(arg0) => Self::WrongSubjectsOf(arg0.clone()),
            Self::WrongObjectsOf(arg0) => Self::WrongObjectsOf(arg0.clone()),
            Self::WrongImplicitClass(arg0) => Self::WrongImplicitClass(arg0.clone()),
        }
    }
}

impl<RDF: Rdf> PartialEq for Target<RDF> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Node(l0), Self::Node(r0)) => l0 == r0,
            (Self::Class(l0), Self::Class(r0)) => l0 == r0,
            (Self::SubjectsOf(l0), Self::SubjectsOf(r0)) => l0 == r0,
            (Self::ObjectsOf(l0), Self::ObjectsOf(r0)) => l0 == r0,
            (Self::ImplicitClass(l0), Self::ImplicitClass(r0)) => l0 == r0,
            (Self::WrongNode(l0), Self::WrongNode(r0)) => l0 == r0,
            (Self::WrongClass(l0), Self::WrongClass(r0)) => l0 == r0,
            (Self::WrongSubjectsOf(l0), Self::WrongSubjectsOf(r0)) => l0 == r0,
            (Self::WrongObjectsOf(l0), Self::WrongObjectsOf(r0)) => l0 == r0,
            (Self::WrongImplicitClass(l0), Self::WrongImplicitClass(r0)) => l0 == r0,
            _ => false,
        }
    }
}
