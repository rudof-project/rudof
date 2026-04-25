use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct TestManifestVocab;

impl RdfVocabulary for TestManifestVocab {
    const BASE: &'static str = "http://www.w3.org/2001/sw/DataAccess/tests/test-manifest#";
}

vocab_term!(TestManifestVocab, MF_ACTION, "action");
vocab_term!(TestManifestVocab, MF_RESULT, "result");
vocab_term!(TestManifestVocab, MF_ENTRIES, "entries");
vocab_term!(TestManifestVocab, MF_INCLUDE, "include");
