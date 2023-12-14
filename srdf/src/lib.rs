pub mod async_srdf;
pub mod bnode;
pub mod lang;
pub mod literal;
pub mod neighs;
pub mod numeric_literal;
pub mod rdf;
pub mod shacl_path;
pub mod srdf;
pub mod srdf_comparisons;
pub mod srdf_parser;

pub use crate::async_srdf::*;
pub use crate::neighs::*;
pub use crate::srdf::*;
pub use crate::srdf_comparisons::*;
pub use bnode::*;
use iri_s::IriS;
use lazy_static::lazy_static;
pub use rdf::*;
pub use shacl_path::*;
pub use srdf_parser::*;

lazy_static! {
    pub static ref RDF: IriS = IriS::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#");
    pub static ref XSD: IriS = IriS::new_unchecked("http://www.w3.org/2001/XMLSchema#");
    pub static ref RDF_TYPE: IriS =
        IriS::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#type");
}
