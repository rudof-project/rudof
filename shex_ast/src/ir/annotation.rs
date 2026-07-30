use rudof_iri::IriS;
use serde::{Deserialize, Serialize};

use super::object_value::ObjectValue;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Annotation {
    predicate: IriS,
    object: ObjectValue,
}
