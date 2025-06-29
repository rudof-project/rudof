use super::compiled_shacl_error::CompiledShaclError;
use iri_s::IriS;
use shacl_ast::target::Target;
use srdf::{RDFNode, Rdf};

/// Represents compiled target declarations
#[derive(Debug, Clone)]
pub enum CompiledTarget {
    Node(RDFNode),
    Class(RDFNode),
    SubjectsOf(IriS),
    ObjectsOf(IriS),
    ImplicitClass(RDFNode),
    // The following target declarations always return violation errors
    WrongTargetNode(RDFNode),
    WrongTargetClass(RDFNode),
    WrongSubjectsOf(RDFNode),
    WrongObjectsOf(RDFNode),
    WrongImplicitClass(RDFNode),
}

impl CompiledTarget {
    pub fn compile<S: Rdf>(target: Target<S>) -> Result<Self, CompiledShaclError> {
        let ans = match target {
            Target::TargetNode(object) => CompiledTarget::Node(object),
            Target::TargetClass(object) => CompiledTarget::Class(object),
            Target::TargetSubjectsOf(iri_ref) => CompiledTarget::SubjectsOf(iri_ref.into()),
            Target::TargetObjectsOf(iri_ref) => CompiledTarget::ObjectsOf(iri_ref.into()),
            Target::TargetImplicitClass(object) => CompiledTarget::ImplicitClass(object),
            Target::WrongTargetNode(_) => todo!(),
            Target::WrongTargetClass(_) => todo!(),
            Target::WrongSubjectsOf(_) => todo!(),
            Target::WrongObjectsOf(_) => todo!(),
            Target::WrongImplicitClass(_) => todo!(),
        };

        Ok(ans)
    }
}
