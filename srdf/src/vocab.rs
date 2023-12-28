use const_format::concatcp;
use iri_s::IriS;

use lazy_static::lazy_static;

pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
pub const RDF_TYPE_STR: &str = concatcp!(RDF, "type");
pub const RDF_FIRST_STR: &str = concatcp!(RDF, "first");
pub const RDF_REST_STR: &str = concatcp!(RDF, "rest");
pub const RDF_NIL_STR: &str = concatcp!(RDF, "nil");
pub const XSD_BOOLEAN_STR: &str = concatcp!(XSD, "boolean");
pub const XSD_INTEGER_STR: &str = concatcp!(XSD, "integer");
pub const XSD_DECIMAL_STR: &str = concatcp!(XSD, "decimal");
pub const XSD_DOUBLE_STR: &str = concatcp!(XSD, "double");

lazy_static! {
    pub static ref RDF_TYPE: IriS = IriS::new_unchecked(RDF_TYPE_STR);
    pub static ref RDF_FIRST: IriS = IriS::new_unchecked(RDF_FIRST_STR);
    pub static ref RDF_REST: IriS = IriS::new_unchecked(RDF_REST_STR);
    pub static ref RDF_NIL: IriS = IriS::new_unchecked(RDF_NIL_STR);
    pub static ref XSD_BOOLEAN: IriS = IriS::new_unchecked(XSD_BOOLEAN_STR);
    pub static ref XSD_INTEGER: IriS = IriS::new_unchecked(XSD_INTEGER_STR);
    pub static ref XSD_DECIMAL: IriS = IriS::new_unchecked(XSD_DECIMAL_STR);
    pub static ref XSD_DOUBLE: IriS = IriS::new_unchecked(XSD_DOUBLE_STR);
}

