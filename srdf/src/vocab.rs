use const_format::concatcp;
use iri_s::IriS;
use iri_s::iri_once;

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

iri_once!(rdf_type, RDF_TYPE_STR);
iri_once!(rdf_first, RDF_FIRST_STR);
iri_once!(rdf_rest, RDF_REST_STR);
iri_once!(rdf_nil, RDF_NIL_STR);

iri_once!(rdfs_label, RDFS_LABEL_STR);
iri_once!(rdfs_subclass_of, RDFS_SUBCLASS_OF_STR);
iri_once!(rdfs_class, RDFS_CLASS_STR);

iri_once!(xsd_boolean, XSD_BOOLEAN_STR);
iri_once!(xsd_integer, XSD_INTEGER_STR);
iri_once!(xsd_decimal, XSD_DECIMAL_STR);
iri_once!(xsd_double, XSD_DOUBLE_STR);
