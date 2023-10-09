use serde_derive::{Deserialize, Serialize};

use super::{iri_ref::IriRef, object_value::ObjectValue};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Annotation {
    predicate: IriRef,
    object: ObjectValue,
}
