use crate::rdf_core::vocabs::RdfVocabulary;
use crate::vocab_term;

pub struct ShexRVocab;

impl RdfVocabulary for ShexRVocab {
    const BASE: &'static str = "http://www.w3.org/ns/shex#";
}

// Classes
vocab_term!(ShexRVocab, SX, "");
vocab_term!(ShexRVocab, SX_ANNOTATION, "Annotation");
vocab_term!(ShexRVocab, SX_EACH_OF, "EachOf");
vocab_term!(ShexRVocab, SX_IRI_STEM, "IriStem");
vocab_term!(ShexRVocab, SX_IRI_STEM_RANGE, "IriStemRange");
vocab_term!(ShexRVocab, SX_LANGUAGE_STEM, "LanguageStem");
vocab_term!(ShexRVocab, SX_LANGUAGE_STEM_RANGE, "LanguageStemRange");
vocab_term!(ShexRVocab, SX_LIETRAL_STEM, "LiteralStem");
vocab_term!(ShexRVocab, SX_LIETRAL_STEM_RANGE, "LiteralStemRange");
vocab_term!(ShexRVocab, SX_NODE_CONSTRAINT, "NodeConstraint");
vocab_term!(ShexRVocab, SX_ONE_OF, "OneOf");
vocab_term!(ShexRVocab, SX_SCHEMA, "Schema");
vocab_term!(ShexRVocab, SX_SEM_ACT, "SemAct");
vocab_term!(ShexRVocab, SX_SHAPE, "Shape");
vocab_term!(ShexRVocab, SX_SHAPE_AND, "ShapeAnd");
vocab_term!(ShexRVocab, SX_SHAPE_EXTERNAL, "ShapeExternal");
vocab_term!(ShexRVocab, SX_SHAPE_NOT, "ShapeNot");
vocab_term!(ShexRVocab, SX_SHAPE_DECL, "ShapeDecl");
vocab_term!(ShexRVocab, SX_SHAPE_OR, "ShapeOr");
vocab_term!(ShexRVocab, SX_TRIPLE_CONSTRAINT, "TripleConstraint");
vocab_term!(ShexRVocab, SX_WILDCARD, "Wildcard");
vocab_term!(ShexRVocab, SX_INF, "INF");

// Properties
vocab_term!(ShexRVocab, SX_ABSTRACT, "abstract");
vocab_term!(ShexRVocab, SX_ANNOTATION_PROP, "annotation");
vocab_term!(ShexRVocab, SX_BNODE, "bnode");
vocab_term!(ShexRVocab, SX_CODE, "code");
vocab_term!(ShexRVocab, SX_CLOSED, "closed");
vocab_term!(ShexRVocab, SX_DATATYPE, "datatype");
vocab_term!(ShexRVocab, SX_EXTRA, "extra");
vocab_term!(ShexRVocab, SX_EXCLUSION, "exclusion");
vocab_term!(ShexRVocab, SX_EXPRESION, "expresion");
vocab_term!(ShexRVocab, SX_EXPRESIONS, "expresions");
vocab_term!(ShexRVocab, SX_FRACTIONDIGITS, "fractiondigits");
vocab_term!(ShexRVocab, SX_FLAGS, "flags");
vocab_term!(ShexRVocab, SX_IRI, "iri");
vocab_term!(ShexRVocab, SX_INVERSE, "inverse");
vocab_term!(ShexRVocab, SX_LENGTH, "length");
vocab_term!(ShexRVocab, SX_LITERAL, "literal");
vocab_term!(ShexRVocab, SX_MIN, "min");
vocab_term!(ShexRVocab, SX_MININCLUSIVE, "mininclusive");
vocab_term!(ShexRVocab, SX_MINEXCLUSIVE, "minexclusive");
vocab_term!(ShexRVocab, SX_MINLENGTH, "minlength");
vocab_term!(ShexRVocab, SX_MAX, "max");
vocab_term!(ShexRVocab, SX_MAXINCLUSIVE, "maxinclusive");
vocab_term!(ShexRVocab, SX_MAXEXCLUSIVE, "maxexclusive");
vocab_term!(ShexRVocab, SX_MAXLENGTH, "maxlength");
vocab_term!(ShexRVocab, SX_NAME, "name");
vocab_term!(ShexRVocab, SX_NEGATED, "negated");
vocab_term!(ShexRVocab, SX_NODE_KIND, "nodeKind");
vocab_term!(ShexRVocab, SX_NON_LITERAL, "nonLiteral");
vocab_term!(ShexRVocab, SX_OBJECT, "object");
vocab_term!(ShexRVocab, SX_PATTERN, "pattern");
vocab_term!(ShexRVocab, SX_PREDICATE, "predicate");
vocab_term!(ShexRVocab, SX_SEM_ACTS, "semActs");
vocab_term!(ShexRVocab, SX_START_ACTS, "startActs");
vocab_term!(ShexRVocab, SX_START, "start");
vocab_term!(ShexRVocab, SX_SHAPES, "shapes");
vocab_term!(ShexRVocab, SX_SHAPE_EXPR, "shapeExpr");
vocab_term!(ShexRVocab, SX_SHAPE_EXPRS, "shapeExprs");
vocab_term!(ShexRVocab, SX_STEM, "stem");
vocab_term!(ShexRVocab, SX_STEM_RANGE, "stemRange");
vocab_term!(ShexRVocab, SX_TOTALDIGITS, "totaldigits");
vocab_term!(ShexRVocab, SX_VALUE_EXPR, "valueExpr");
vocab_term!(ShexRVocab, SX_VALUES, "values");