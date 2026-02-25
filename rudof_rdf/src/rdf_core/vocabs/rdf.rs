use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct RdfVocab;

/// RDF vocabulary terms
impl RdfVocabulary for RdfVocab {
    const BASE: &'static str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
}

vocab_term!(RdfVocab, RDF_REIFIES, "reifies");
vocab_term!(RdfVocab, RDF_TYPE, "type");
vocab_term!(RdfVocab, RDF_LANG_STRING, "langString");
vocab_term!(RdfVocab, RDF_FIRST, "first");
vocab_term!(RdfVocab, RDF_REST, "rest");
vocab_term!(RdfVocab, RDF_NIL, "nil");
