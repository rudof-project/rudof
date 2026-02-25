use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct XsdVocab;

/// XSD vocabulary terms
impl RdfVocabulary for XsdVocab {
    const BASE: &'static str = "http://www.w3.org/2001/XMLSchema#";
}

vocab_term!(XsdVocab, XSD_BOOLEAN, "boolean");
vocab_term!(XsdVocab, XSD_STRING, "string");
vocab_term!(XsdVocab, XSD_DATE_TIME, "dateTime");
vocab_term!(XsdVocab, XSD_INTEGER, "integer");
vocab_term!(XsdVocab, XSD_NEGATIVE_INTEGER, "negativeInteger");
vocab_term!(XsdVocab, XSD_POSITIVE_INTEGER, "positiveInteger");
vocab_term!(XsdVocab, XSD_NON_NEGATIVE_INTEGER, "nonNegativeInteger");
vocab_term!(XsdVocab, XSD_NON_POSITIVE_INTEGER, "nonPositiveInteger");
vocab_term!(XsdVocab, XSD_DECIMAL, "decimal");
vocab_term!(XsdVocab, XSD_DOUBLE, "double");
vocab_term!(XsdVocab, XSD_LONG, "long");
vocab_term!(XsdVocab, XSD_BYTE, "byte");
vocab_term!(XsdVocab, XSD_FLOAT, "float");
vocab_term!(XsdVocab, XSD_SHORT, "short");
vocab_term!(XsdVocab, XSD_UNSIGNED_INT, "unsignedInt");
vocab_term!(XsdVocab, XSD_UNSIGNED_LONG, "unsignedLong");
vocab_term!(XsdVocab, XSD_UNSIGNED_SHORT, "unsignedShort");
vocab_term!(XsdVocab, XSD_UNSIGNED_BYTE, "unsignedByte");
