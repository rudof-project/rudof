//! EARL 1.0 reports.
//!
//! [EARL](https://www.w3.org/TR/EARL10-Schema/) (Evaluation and Report
//! Language) is the vocabulary the W3C uses to collect implementation reports:
//! one `earl:Assertion` per test, naming what was tested, against which
//! criterion, and with what outcome.

mod error;
pub mod report;
mod software;
pub mod model;

pub use error::EarlError;
use rudof_rdf::rdf_core::BuildRDF;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::vocabs::{DctVocab, DoapVocab, EarlVocab, FoafVocab};
pub use software::{Release, Software};
use crate::error::graph_error;

pub(crate) fn earl_prefixes() -> [(&'static str, rudof_iri::IriS); 4] {
    [
        ("earl", EarlVocab::earl()),
        ("doap", DoapVocab::doap()),
        ("foaf", FoafVocab::foaf()),
        ("dct", DctVocab::dct()),
    ]
}

pub(crate) fn new_subject<RDF: BuildRDF>(writer: &mut RDF) -> Result<RDF::Subject, EarlError> {
    Ok(writer
        .add_bnode()
        .map_err(graph_error::<RDF>("Error creating a blank node"))?
        .into())
}

/// Reads an [`Object`] as a subject
pub(crate) fn subject_for<RDF: BuildRDF>(object: &Object) -> Result<RDF::Subject, EarlError> {
    RDF::Subject::try_from(object.clone())
        .map_err(|_| EarlError::CastError(format!("term ({object})"), "subject".to_string()))
}
