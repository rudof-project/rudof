use std::str::FromStr;

use const_format::concatcp;
use iri_s::IriS;
use lazy_static::lazy_static;

pub const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
pub const RDFS: &str = "http://www.w3.org/2000/01/rdf-schema#";
pub const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
pub const RDF_TYPE_STR: &str = concatcp!(RDF, "type");
pub const RDF_FIRST_STR: &str = concatcp!(RDF, "first");
pub const RDF_REST_STR: &str = concatcp!(RDF, "rest");
pub const RDF_NIL_STR: &str = concatcp!(RDF, "nil");
pub const RDFS_LABEL_STR: &str = concatcp!(RDFS, "label");
pub const RDFS_SUBCLASS_OF_STR: &str = concatcp!(RDFS, "subClassOf");
pub const RDFS_CLASS_STR: &str = concatcp!(RDFS, "Class");
pub const XSD_BOOLEAN_STR: &str = concatcp!(XSD, "boolean");
pub const XSD_INTEGER_STR: &str = concatcp!(XSD, "integer");
pub const XSD_DECIMAL_STR: &str = concatcp!(XSD, "decimal");
pub const XSD_DOUBLE_STR: &str = concatcp!(XSD, "double");

lazy_static! {
    pub static ref RDF_TYPE: IriS = IriS::from_str(RDF_TYPE_STR).unwrap();
    pub static ref RDF_FIRST: IriS = IriS::from_str(RDF_FIRST_STR).unwrap();
    pub static ref RDF_REST: IriS = IriS::from_str(RDF_REST_STR).unwrap();
    pub static ref RDF_NIL: IriS = IriS::from_str(RDF_NIL_STR).unwrap();
    pub static ref RDFS_LABEL: IriS = IriS::from_str(RDFS_LABEL_STR).unwrap();
    pub static ref RDFS_SUBCLASS_OF: IriS = IriS::from_str(RDFS_SUBCLASS_OF_STR).unwrap();
    pub static ref RDFS_CLASS: IriS = IriS::from_str(RDFS_CLASS_STR).unwrap();
    pub static ref XSD_BOOLEAN: IriS = IriS::from_str(XSD_BOOLEAN_STR).unwrap();
    pub static ref XSD_INTEGER: IriS = IriS::from_str(XSD_INTEGER_STR).unwrap();
    pub static ref XSD_DECIMAL: IriS = IriS::from_str(XSD_DECIMAL_STR).unwrap();
    pub static ref XSD_DOUBLE: IriS = IriS::from_str(XSD_DOUBLE_STR).unwrap();
}
