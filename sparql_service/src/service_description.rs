//! A set whose elements can be repeated. The set tracks how many times each element appears
//!

use std::path::Path;

use iri_s::IriS;

use crate::ServiceDescriptionError;
#[derive(Clone, PartialEq, Eq, Default)]
pub struct ServiceDescription {
    endpoint: IriS,
}

impl ServiceDescription {
    pub fn from_path<P: AsRef<Path>>(
        path: P,
    ) -> Result<ServiceDescription, ServiceDescriptionError> {
        todo!()
    }
}
