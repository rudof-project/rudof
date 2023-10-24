use iri_s::IriSError;
use prefixmap::PrefixMap;
use serde_derive::{Deserialize, Serialize};

use crate::{Deref, DerefError};

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

impl Deref for Annotation {
    fn deref(&self, base: &Option<iri_s::IriS>, prefixmap: &Option<prefixmap::PrefixMap>) -> Result<Self, DerefError> {
        let new_pred = self.predicate.deref(base, prefixmap)?;
        let new_obj = self.object.deref(base, prefixmap)?;
        Ok(Annotation {
            predicate: new_pred,
            object: new_obj
        })
    }
}
