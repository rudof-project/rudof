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
pub const RDF_REIFIES_STR: &str = concatcp!(RDF, "reifies");
pub const SH_STR: &str = "http://www.w3.org/ns/shacl#";

// The following constants are required for SHACL Path parsing
pub const SH_ALTERNATIVE_PATH_STR: &str = concatcp!(SH_STR, "alternativePath");
pub const SH_ZERO_OR_ONE_PATH_STR: &str = concatcp!(SH_STR, "zeroOrOnePath");
pub const SH_ZERO_OR_MORE_PATH_STR: &str = concatcp!(SH_STR, "zeroOrMorePath");
pub const SH_ONE_OR_MORE_PATH_STR: &str = concatcp!(SH_STR, "oneOrMorePath");
pub const SH_INVERSE_PATH_STR: &str = concatcp!(SH_STR, "inversePath");

iri_once!(rdf_type, RDF_TYPE_STR);
iri_once!(rdf_reifies, RDF_REIFIES_STR);
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

iri_once!(sh_alternative_path, SH_ALTERNATIVE_PATH_STR);
iri_once!(sh_zero_or_one_path, SH_ZERO_OR_ONE_PATH_STR);
iri_once!(sh_zero_or_more_path, SH_ZERO_OR_MORE_PATH_STR);
iri_once!(sh_one_or_more_path, SH_ONE_OR_MORE_PATH_STR);
iri_once!(sh_inverse_path, SH_INVERSE_PATH_STR);
