use super::compiled_shacl_error::CompiledShaclError;
use iri_s::IriS;
use shacl_ast::target::Target;
use srdf::RDFNode;

#[derive(Debug)]
pub enum CompiledTarget {
    Node(RDFNode),
    Class(RDFNode),
    SubjectsOf(IriS),
    ObjectsOf(IriS),
    ImplicitClass(RDFNode),
}

impl CompiledTarget {
    pub fn compile(target: Target) -> Result<Self, CompiledShaclError> {
        let ans = match target {
            Target::TargetNode(object) => CompiledTarget::Node(object.into()),
            Target::TargetClass(object) => CompiledTarget::Class(object),
            Target::TargetSubjectsOf(iri_ref) => CompiledTarget::SubjectsOf(iri_ref.into()),
            Target::TargetObjectsOf(iri_ref) => CompiledTarget::ObjectsOf(iri_ref.into()),
            Target::TargetImplicitClass(object) => CompiledTarget::ImplicitClass(object),
        };

        Ok(ans)
    }
}
