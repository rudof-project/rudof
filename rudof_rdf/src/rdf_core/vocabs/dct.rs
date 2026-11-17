use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

/// DCT (Dublin Core Terms) vocabulary terms
pub struct DctVocab;

impl RdfVocabulary for DctVocab {
    const BASE: &'static str = "http://purl.org/dc/terms/";
}

vocab_term!(DctVocab, DCT, "");
vocab_term!(DctVocab, DCT_ISSUED, "issued");
vocab_term!(DctVocab, DCT_DATE, "date");
vocab_term!(DctVocab, DCT_TITLE, "title");
vocab_term!(DctVocab, DCT_DESCRIPTION, "description");
