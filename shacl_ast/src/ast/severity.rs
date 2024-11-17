use iri_s::iri;
use iri_s::IriS;
use srdf::model::rdf::TPredicate;
use srdf::model::rdf::Rdf;
use srdf::model::Iri;

use crate::vocab::SH_INFO_STR;
use crate::vocab::SH_VIOLATION_STR;
use crate::vocab::SH_WARNING_STR;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Severity<R: Rdf> {
    Violation,
    Warning,
    Info,
    Generic(TPredicate<R>),
}

impl<R: Rdf> From<&Severity<R>> for IriS {
    fn from(value: &Severity<R>) -> Self {
        match value {
            Severity::Violation => iri!(SH_VIOLATION_STR),
            Severity::Warning => iri!(SH_WARNING_STR),
            Severity::Info => iri!(SH_INFO_STR),
            Severity::Generic(iri) => iri.as_iri_s(),
        }
    }
}
