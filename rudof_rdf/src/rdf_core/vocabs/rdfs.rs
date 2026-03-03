use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct RdfsVocab;

/// RDFS vocabulary terms.
impl RdfVocabulary for RdfsVocab {
    const BASE: &'static str = "http://www.w3.org/2000/01/rdf-schema#";
}

vocab_term!(RdfsVocab, RDFS_LABEL, "label");
vocab_term!(RdfsVocab, RDFS_CLASS, "Class");
vocab_term!(RdfsVocab, RDFS_SUBCLASS_OF_STR, "subClassOf");
