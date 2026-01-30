// RDF and XML Schema vocabulary constants and accessors.
//
// This module provides compile-time constants and thread-safe singleton accessors for
// commonly used IRIs from the RDF and XML Schema (XSD) vocabularies. These constants
// represent standard properties and datatypes used throughout RDF processing.

use const_format::concatcp;
use iri_s::{IriS, iri_once};

/// Base namespace for RDF vocabulary terms.
const RDF: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
/// Base namespace for XML Schema datatypes.
const XSD: &str = "http://www.w3.org/2001/XMLSchema#";
/// Base namespace for SHACL terms.
pub const SH: &str = "http://www.w3.org/ns/shacl#";

// --------------------------------------------
// RDF vocabulary IRI accessors
// --------------------------------------------
const RDF_REIFIES_STR: &str = concatcp!(RDF, "reifies");
const RDF_TYPE_STR: &str = concatcp!(RDF, "type");
const RDF_LANG_STRING_STR: &str = concatcp!(RDF, "langString");
const RDF_FIRST_STR: &str = concatcp!(RDF, "first");
const RDF_REST_STR: &str = concatcp!(RDF, "rest");
const RDF_NIL_STR: &str = concatcp!(RDF, "nil");

iri_once!(rdf_type, RDF_TYPE_STR);
iri_once!(rdf_reifies, RDF_REIFIES_STR);
iri_once!(rdf_lang, RDF_LANG_STRING_STR);
iri_once!(rdf_first, RDF_FIRST_STR);
iri_once!(rdf_rest, RDF_REST_STR);
iri_once!(rdf_nil, RDF_NIL_STR);

// --------------------------------------------
// XSD datatype IRI accessors
// --------------------------------------------
const XSD_BOOLEAN_STR: &str = concatcp!(XSD, "boolean");
const XSD_STRING_STR: &str = concatcp!(XSD, "string");
const XSD_DATE_TIME_STR: &str = concatcp!(XSD, "dateTime");
const XSD_INTEGER_STR: &str = concatcp!(XSD, "integer");
const XSD_NEGATIVE_INTEGER_STR: &str = concatcp!(XSD, "negativeInteger");
const XSD_POSITIVE_INTEGER_STR: &str = concatcp!(XSD, "positiveInteger");
const XSD_NON_NEGATIVE_INTEGER_STR: &str = concatcp!(XSD, "nonNegativeInteger");
const XSD_NON_POSITIVE_INTEGER_STR: &str = concatcp!(XSD, "nonPositiveInteger");
const XSD_DECIMAL_STR: &str = concatcp!(XSD, "decimal");
const XSD_DOUBLE_STR: &str = concatcp!(XSD, "double");
const XSD_LONG_STR: &str = concatcp!(XSD, "long");
const XSD_BYTE_STR: &str = concatcp!(XSD, "byte");
const XSD_FLOAT_STR: &str = concatcp!(XSD, "float");
const XSD_SHORT_STR: &str = concatcp!(XSD, "short");
const XSD_UNSIGNED_INT_STR: &str = concatcp!(XSD, "unsignedInt");
const XSD_UNSIGNED_LONG_STR: &str = concatcp!(XSD, "unsignedLong");
const XSD_UNSIGNED_SHORT_STR: &str = concatcp!(XSD, "unsignedShort");
const XSD_UNSIGNED_BYTE_STR: &str = concatcp!(XSD, "unsignedByte");

iri_once!(xsd_boolean, XSD_BOOLEAN_STR);
iri_once!(xsd_string, XSD_STRING_STR);
iri_once!(xsd_date_time, XSD_DATE_TIME_STR);
iri_once!(xsd_integer, XSD_INTEGER_STR);
iri_once!(xsd_non_negative_integer, XSD_NON_NEGATIVE_INTEGER_STR);
iri_once!(xsd_non_positive_integer, XSD_NON_POSITIVE_INTEGER_STR);
iri_once!(xsd_negative_integer, XSD_NEGATIVE_INTEGER_STR);
iri_once!(xsd_positive_integer, XSD_POSITIVE_INTEGER_STR);
iri_once!(xsd_decimal, XSD_DECIMAL_STR);
iri_once!(xsd_double, XSD_DOUBLE_STR);
iri_once!(xsd_long, XSD_LONG_STR);
iri_once!(xsd_byte, XSD_BYTE_STR);
iri_once!(xsd_float, XSD_FLOAT_STR);
iri_once!(xsd_short, XSD_SHORT_STR);
iri_once!(xsd_unsigned_int, XSD_UNSIGNED_INT_STR);
iri_once!(xsd_unsigned_long, XSD_UNSIGNED_LONG_STR);
iri_once!(xsd_unsigned_short, XSD_UNSIGNED_SHORT_STR);
iri_once!(xsd_unsigned_byte, XSD_UNSIGNED_BYTE_STR);

// --------------------------------------------
// SHACL vocabulary IRI accessors
// --------------------------------------------
const SH_ALTERNATIVE_PATH_STR: &str = concatcp!(SH, "alternativePath");
const SH_ZERO_OR_ONE_PATH_STR: &str = concatcp!(SH, "zeroOrOnePath");
const SH_ZERO_OR_MORE_PATH_STR: &str = concatcp!(SH, "zeroOrMorePath");
const SH_ONE_OR_MORE_PATH_STR: &str = concatcp!(SH, "oneOrMorePath");
const SH_INVERSE_PATH_STR: &str = concatcp!(SH, "inversePath");

iri_once!(sh_alternative_path, SH_ALTERNATIVE_PATH_STR);
iri_once!(sh_zero_or_one_path, SH_ZERO_OR_ONE_PATH_STR);
iri_once!(sh_zero_or_more_path, SH_ZERO_OR_MORE_PATH_STR);
iri_once!(sh_one_or_more_path, SH_ONE_OR_MORE_PATH_STR);
iri_once!(sh_inverse_path, SH_INVERSE_PATH_STR);