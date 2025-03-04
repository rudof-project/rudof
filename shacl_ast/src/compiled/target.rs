use srdf::Rdf;

use crate::target::Target;

use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;

#[derive(Clone, Debug)]
pub enum CompiledTarget<S: Rdf> {
    Node(S::Term),
    Class(S::Term),
    SubjectsOf(S::IRI),
    ObjectsOf(S::IRI),
    ImplicitClass(S::Term),
}

impl<S: Rdf> CompiledTarget<S> {
    pub fn compile(target: Target) -> Result<Self, CompiledShaclError> {
        let ans = match target {
            Target::TargetNode(object) => CompiledTarget::Node(object.into()),
            Target::TargetClass(object) => CompiledTarget::Class(object.into()),
            Target::TargetSubjectsOf(iri_ref) => {
                CompiledTarget::SubjectsOf(convert_iri_ref::<S>(iri_ref)?)
            }
            Target::TargetObjectsOf(iri_ref) => {
                CompiledTarget::ObjectsOf(convert_iri_ref::<S>(iri_ref)?)
            }
            Target::TargetImplicitClass(object) => CompiledTarget::ImplicitClass(object.into()),
        };

        Ok(ans)
    }
}
