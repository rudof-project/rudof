use std::fmt::Display;

use super::compiled_shacl_error::CompiledShaclError;
use iri_s::IriS;
use rdf::rdf_core::{Rdf, term::Object};
use shacl_ast::target::Target;

/// Represents compiled target declarations
#[derive(Debug, Clone)]
pub enum CompiledTarget {
    Node(Object),
    Class(Object),
    SubjectsOf(IriS),
    ObjectsOf(IriS),
    ImplicitClass(Object),
    // The following target declarations always return violation errors
    WrongTargetNode(Object),
    WrongTargetClass(Object),
    WrongSubjectsOf(Object),
    WrongObjectsOf(Object),
    WrongImplicitClass(Object),
}

impl CompiledTarget {
    pub fn compile<S: Rdf>(target: Target<S>) -> Result<Self, Box<CompiledShaclError>> {
        let ans = match target {
            Target::Node(object) => CompiledTarget::Node(object),
            Target::Class(object) => CompiledTarget::Class(object),
            Target::SubjectsOf(iri_ref) => CompiledTarget::SubjectsOf(iri_ref.into()),
            Target::ObjectsOf(iri_ref) => CompiledTarget::ObjectsOf(iri_ref.into()),
            Target::ImplicitClass(object) => CompiledTarget::ImplicitClass(object),
            Target::WrongNode(_) => todo!(),
            Target::WrongClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        };

        Ok(ans)
    }
}

impl Display for CompiledTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompiledTarget::Node(node) => write!(f, "TargetNode({node})"),
            CompiledTarget::Class(node) => write!(f, "TargetClass({node})"),
            CompiledTarget::SubjectsOf(iri) => write!(f, "TargetSubjectsOf({iri})"),
            CompiledTarget::ObjectsOf(iri) => write!(f, "TargetObjectsOf({iri})"),
            CompiledTarget::ImplicitClass(node) => write!(f, "TargetImplicitClass({node})"),
            CompiledTarget::WrongTargetNode(node) => write!(f, "WrongTargetNode({node})"),
            CompiledTarget::WrongTargetClass(node) => write!(f, "WrongTargetClass({node})"),
            CompiledTarget::WrongSubjectsOf(node) => write!(f, "WrongSubjectsOf({node})"),
            CompiledTarget::WrongObjectsOf(node) => write!(f, "WrongObjectsOf({node})"),
            CompiledTarget::WrongImplicitClass(node) => {
                write!(f, "WrongImplicitClass({node})")
            },
        }
    }
}
