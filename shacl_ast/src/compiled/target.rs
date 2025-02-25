use srdf::Rdf;

use crate::target::Target;

use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;

#[derive(Debug)]
pub enum CompiledTarget<S: Rdf> {
    TargetNode(S::Term),
    TargetClass(S::Term),
    TargetSubjectsOf(S::IRI),
    TargetObjectsOf(S::IRI),
}

impl<S: Rdf> CompiledTarget<S> {
    pub fn compile(target: Target) -> Result<Self, CompiledShaclError> {
        let ans = match target {
            Target::TargetNode(object) => CompiledTarget::TargetNode(object.into()),
            Target::TargetClass(object) => CompiledTarget::TargetClass(object.into()),
            Target::TargetSubjectsOf(iri_ref) => {
                CompiledTarget::TargetSubjectsOf(convert_iri_ref::<S>(iri_ref)?)
            }
            Target::TargetObjectsOf(iri_ref) => {
                CompiledTarget::TargetObjectsOf(convert_iri_ref::<S>(iri_ref)?)
            }
        };

        Ok(ans)
    }
}
