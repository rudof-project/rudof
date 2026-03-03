use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct ShaclNodeExprVocab;

impl RdfVocabulary for ShaclNodeExprVocab {
    const BASE: &'static str = "http://www.w3.org/ns/shacl-node-expr#";
}

// Basic Node Expressions
vocab_term!(ShaclNodeExprVocab, SHNEX_VAR, "var");
vocab_term!(ShaclNodeExprVocab, SHNEX_PATH_VALUES, "pathValues");
vocab_term!(ShaclNodeExprVocab, SHNEX_FOCUS_NODE, "focusNode");
vocab_term!(ShaclNodeExprVocab, SHNEX_EXISTS, "exists");
vocab_term!(ShaclNodeExprVocab, SHNEX_IF, "if");
vocab_term!(ShaclNodeExprVocab, SHNEX_THEN, "then");
vocab_term!(ShaclNodeExprVocab, SHNEX_ELSE, "else");

// List Operator Expressions
vocab_term!(ShaclNodeExprVocab, SHNEX_DISTINCT, "distinct");
vocab_term!(ShaclNodeExprVocab, SHNEX_INTERSECTION, "intersection");
vocab_term!(ShaclNodeExprVocab, SHNEX_CONCAT, "concat");
vocab_term!(ShaclNodeExprVocab, SHNEX_REMOVE, "remove");
vocab_term!(ShaclNodeExprVocab, SHNEX_FILTER_SHAPE, "filterShape");
vocab_term!(ShaclNodeExprVocab, SHNEX_LIMIT, "limit");
vocab_term!(ShaclNodeExprVocab, SHNEX_OFFSET, "offset");

// Advanced Sequence Operations
vocab_term!(ShaclNodeExprVocab, SHNEX_FLAT_MAP, "flatMap");
vocab_term!(ShaclNodeExprVocab, SHNEX_FIND_FIRST, "findFirst");
vocab_term!(ShaclNodeExprVocab, SHNEX_MATCH_ALL, "matchAll");

// Aggregation Expressions
vocab_term!(ShaclNodeExprVocab, SHNEX_COUNT, "count");
vocab_term!(ShaclNodeExprVocab, SHNEX_MIN, "min");
vocab_term!(ShaclNodeExprVocab, SHNEX_MAX, "max");
vocab_term!(ShaclNodeExprVocab, SHNEX_SUM, "sum");

// Miscellaneous Node Expressions
vocab_term!(ShaclNodeExprVocab, SHNEX_INSTANCES_OF, "instancesOf");
vocab_term!(ShaclNodeExprVocab, SHNEX_NODES_MATCHING, "nodesMatching");

vocab_term!(ShaclNodeExprVocab, SHNEX_NODES, "nodes");
