use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct ShaclTestVocab;

impl RdfVocabulary for ShaclTestVocab {
    const BASE: &'static str = "http://www.w3.org/ns/shacl-test#";
}

vocab_term!(ShaclTestVocab, SHT_DATA_GRAPH, "dataGraph");
vocab_term!(ShaclTestVocab, SHT_SHAPES_GRAPH, "shapesGraph");
vocab_term!(ShaclTestVocab, SHT_FAILURE, "Failure");
vocab_term!(ShaclTestVocab, SHT_NODE_EXPR, "nodeExpr");
vocab_term!(ShaclTestVocab, SHT_FOCUS_NODE, "focusNode");
vocab_term!(ShaclTestVocab, SHT_IGNORE_ORDER, "ignoreOrder");
vocab_term!(ShaclTestVocab, SHT_SCOPE_PREFIX, "scope-");
