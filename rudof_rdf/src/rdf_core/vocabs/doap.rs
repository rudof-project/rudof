use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

/// DOAP (Description of a Project) vocabulary terms
pub struct DoapVocab;

impl RdfVocabulary for DoapVocab {
    const BASE: &'static str = "http://usefulinc.com/ns/doap#";
}

vocab_term!(DoapVocab, DOAP, "");
vocab_term!(DoapVocab, DOAP_PROJECT, "Project");
vocab_term!(DoapVocab, DOAP_VERSION, "Version");
vocab_term!(DoapVocab, DOAP_NAME, "name");
vocab_term!(DoapVocab, DOAP_DESCRIPTION, "description");
vocab_term!(DoapVocab, DOAP_DEVELOPER, "developer");
vocab_term!(DoapVocab, DOAP_HOMEPAGE, "homepage");
vocab_term!(DoapVocab, DOAP_RELEASE, "release");
vocab_term!(DoapVocab, DOAP_REVISION, "revision");
vocab_term!(DoapVocab, DOAP_CREATED, "created");
vocab_term!(DoapVocab, DOAP_PROGRAMMING_LANGUAGE, "programming-language");
