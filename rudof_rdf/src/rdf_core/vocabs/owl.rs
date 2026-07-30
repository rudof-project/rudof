use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

/// Owl vocabulary terms
pub struct OwlVocab;

impl RdfVocabulary for OwlVocab {
    const BASE: &'static str = "http://www.w3.org/2002/07/owl#";
}

vocab_term!(OwlVocab, OWL, "");
vocab_term!(OwlVocab, OWL_ONTOLOGY, "Ontology");

vocab_term!(OwlVocab, OWL_IMPORTS, "imports");
