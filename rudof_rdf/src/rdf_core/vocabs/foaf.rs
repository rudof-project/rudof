use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

/// FOAF vocabulary terms
pub struct FoafVocab;

impl RdfVocabulary for FoafVocab {
    const BASE: &'static str = "http://xmlns.com/foaf/0.1/";
}

vocab_term!(FoafVocab, FOAF, "");
vocab_term!(FoafVocab, FOAF_PERSON, "Person");
vocab_term!(FoafVocab, FOAF_ORGANIZATION, "Organization");
vocab_term!(FoafVocab, FOAF_AGENT, "Agent");
vocab_term!(FoafVocab, FOAF_NAME, "name");
vocab_term!(FoafVocab, FOAF_HOMEPAGE, "homepage");
vocab_term!(FoafVocab, FOAF_MAKER, "maker");
vocab_term!(FoafVocab, FOAF_PRIMARY_TOPIC, "primaryTopic");
