use srdf::SRDFBasic;

use crate::target::Target;

use super::compiled_shacl_error::CompiledShaclError;
use super::convert_iri_ref;

pub enum CompiledTarget<S: SRDFBasic> {
    TargetNode(S::Term),
    TargetClass(S::Term),
    TargetSubjectsOf(S::IRI),
    TargetObjectsOf(S::IRI),
}

impl<S: SRDFBasic> CompiledTarget<S> {
    pub fn compile(target: Target) -> Result<Self, CompiledShaclError> {
        let ans = match target {
            Target::TargetNode(object) => CompiledTarget::TargetNode(S::object_as_term(&object)),
            Target::TargetClass(object) => CompiledTarget::TargetClass(S::object_as_term(&object)),
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
