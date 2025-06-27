use super::compiled_shacl_error::CompiledShaclError;
use iri_s::IriS;
use shacl_ast::target::Target;
use srdf::{RDFNode, Rdf};

/// Represents compiled target declarations
#[derive(Debug)]
pub enum CompiledTarget<S: Rdf> {
    Node(RDFNode),
    Class(RDFNode),
    SubjectsOf(IriS),
    ObjectsOf(IriS),
    ImplicitClass(RDFNode),
    // The following target declarations always return violation errors 
    WrongTargetNode(S::Term),
    WrongTargetClass(S::Term),
    WrongSubjectsOf(S::Term),
    WrongObjectsOf(S::Term),
    WrongImplicitClass(S::Term)
}

impl<S: Rdf> CompiledTarget<S> {
    pub fn compile(target: Target<S>) -> Result<Self, CompiledShaclError> {
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
