use rudof_iri::IriS;

use super::object_value::ObjectValue;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Annotation {
    predicate: IriS,
    object: ObjectValue,
}
