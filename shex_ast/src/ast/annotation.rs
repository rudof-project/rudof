use serde_derive::{Deserialize, Serialize};

use super::{iri_ref::IriRef, object_value::ObjectValue};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct Annotation {
    predicate: IriRef,
    object: ObjectValue,
}

impl Annotation {
    pub fn new(predicate: IriRef, object: ObjectValue) -> Annotation {
        Annotation { predicate, object }
    }
}
