use const_format::concatcp;
use iri_s::IriS;
use std::sync::OnceLock;

// TODO - Move this to a vocab / utils crate

#[macro_export]
macro_rules! vocab_term {
    ($voc:ident, $name:ident, $suffix:literal) => {
        impl $voc {
            pub const $name: &'static str = concatcp!($voc::BASE, $suffix);

            paste::paste! {
                pub fn [<$name:lower>]() -> &'static IriS {
                    static IRI: OnceLock<IriS> = OnceLock::new();
                    IRI.get_or_init(|| IriS::new_unchecked(Self::$name))
                }
            }
        }
    };
}

pub trait RdfVocabulary {
    const BASE: &'static str;

    fn base_iri() -> &'static IriS {
        static IRI: OnceLock<IriS> = OnceLock::new();
        IRI.get_or_init(|| IriS::new_unchecked(Self::BASE))
    }
}

pub struct ShaclVocab;

impl RdfVocabulary for ShaclVocab {
    const BASE: &'static str = "http://www.w3.org/ns/shacl#";
}

vocab_term!(ShaclVocab, SH, "");
vocab_term!(ShaclVocab, SH_BLANK_NODE, "BlankNode");
vocab_term!(ShaclVocab, SH_BLANK_NODE_OR_IRI, "BlankNodeOrIRI");
vocab_term!(ShaclVocab, SH_BLANK_NODE_OR_LITERAL, "BlankNodeOrLiteral");
vocab_term!(ShaclVocab, SH_DEBUG, "Debug");
vocab_term!(ShaclVocab, SH_INFO, "Info");
vocab_term!(ShaclVocab, SH_IRI, "IRI");
vocab_term!(ShaclVocab, SH_IRI_OR_LITERAL, "IRIOrLiteral");
vocab_term!(ShaclVocab, SH_LITERAL, "Literal");
vocab_term!(ShaclVocab, SH_NODE_SHAPE, "NodeShape");
vocab_term!(ShaclVocab, SH_PROPERTY_SHAPE, "PropertyShape");
vocab_term!(ShaclVocab, SH_SHAPE, "Shape");
vocab_term!(ShaclVocab, SH_SCHEMA, "Schema");
vocab_term!(ShaclVocab, SH_VALIDATION_REPORT, "ValidationReport");
vocab_term!(ShaclVocab, SH_VALIDATION_RESULT, "ValidationResult");
vocab_term!(ShaclVocab, SH_TRACE, "Trace");
vocab_term!(ShaclVocab, SH_VIOLATION, "Violation");
vocab_term!(ShaclVocab, SH_WARNING, "Warning");
vocab_term!(ShaclVocab, SH_AND, "and");
vocab_term!(ShaclVocab, SH_CLASS, "class");
vocab_term!(ShaclVocab, SH_CLOSED, "closed");
vocab_term!(ShaclVocab, SH_CONFORMS, "conforms");
vocab_term!(ShaclVocab, SH_DATATYPE, "datatype");
vocab_term!(ShaclVocab, SH_DEACTIVATED, "deactivated");
vocab_term!(ShaclVocab, SH_DESCRIPTION, "description");
vocab_term!(ShaclVocab, SH_DISJOINT, "disjoint");
vocab_term!(ShaclVocab, SH_EQUALS, "equals");
vocab_term!(ShaclVocab, SH_ENTAILMENT, "entailment");
vocab_term!(ShaclVocab, SH_FLAGS, "flags");
vocab_term!(ShaclVocab, SH_FOCUS_NODE, "focusNode");
vocab_term!(ShaclVocab, SH_GROUP, "group");
vocab_term!(ShaclVocab, SH_HAS_VALUE, "hasValue");
vocab_term!(ShaclVocab, SH_IGNORED_PROPERTIES, "ignoredProperties");
vocab_term!(ShaclVocab, SH_IN, "in");
vocab_term!(ShaclVocab, SH_LANGUAGE_IN, "languageIn");
vocab_term!(ShaclVocab, SH_LESS_THAN, "lessThan");
vocab_term!(ShaclVocab, SH_LESS_THAN_OR_EQUALS, "lessThanOrEquals");
vocab_term!(ShaclVocab, SH_MIN_COUNT, "minCount");
vocab_term!(ShaclVocab, SH_MAX_COUNT, "maxCount");
vocab_term!(ShaclVocab, SH_MIN_INCLUSIVE, "minInclusive");
vocab_term!(ShaclVocab, SH_MIN_EXCLUSIVE, "minExclusive");
vocab_term!(ShaclVocab, SH_MAX_INCLUSIVE, "maxInclusive");
vocab_term!(ShaclVocab, SH_MAX_EXCLUSIVE, "maxExclusive");
vocab_term!(ShaclVocab, SH_MIN_LENGTH, "minLength");
vocab_term!(ShaclVocab, SH_MAX_LENGTH, "maxLength");
vocab_term!(ShaclVocab, SH_MESSAGE, "message");
vocab_term!(ShaclVocab, SH_NAME, "name");
vocab_term!(ShaclVocab, SH_NODE_KIND, "nodeKind");
vocab_term!(ShaclVocab, SH_NODE, "node");
vocab_term!(ShaclVocab, SH_NOT, "not");
vocab_term!(ShaclVocab, SH_OR, "or");
vocab_term!(ShaclVocab, SH_ORDER, "order");
vocab_term!(ShaclVocab, SH_PATH, "path");
vocab_term!(ShaclVocab, SH_PATTERN, "pattern");
vocab_term!(ShaclVocab, SH_PROPERTY, "property");
vocab_term!(ShaclVocab, SH_QUALIFIED_MIN_COUNT, "qualifiedMinCount");
vocab_term!(ShaclVocab, SH_QUALIFIED_MAX_COUNT, "qualifiedMaxCount");
vocab_term!(ShaclVocab, SH_QUALIFIED_VALUE_SHAPE, "qualifiedValueShape");
vocab_term!(
    ShaclVocab,
    SH_QUALIFIED_VALUE_SHAPES_DISJOINT,
    "qualifiedValueShapesDisjoint"
);
vocab_term!(ShaclVocab, SH_RESULT, "result");
vocab_term!(ShaclVocab, SH_RESULT_PATH, "resultPath");
vocab_term!(ShaclVocab, SH_RESULT_SEVERITY, "resultSeverity");
vocab_term!(ShaclVocab, SH_RESULT_MESSAGE, "resultMessage");
vocab_term!(ShaclVocab, SH_SHAPES_GRAPH, "shapesGraph");
vocab_term!(ShaclVocab, SH_SEVERITY, "severity");
vocab_term!(ShaclVocab, SH_SOURCE_CONSTRAINT_COMPONENT, "sourceConstraintComponent");
vocab_term!(ShaclVocab, SH_SOURCE_SHAPE, "sourceShape");
vocab_term!(ShaclVocab, SH_VALUE, "value");
vocab_term!(ShaclVocab, SH_TARGET_NODE, "targetNode");
vocab_term!(ShaclVocab, SH_TARGET_CLASS, "targetClass");
vocab_term!(ShaclVocab, SH_TARGET_SUBJECTS_OF, "targetSubjectsOf");
vocab_term!(ShaclVocab, SH_TARGET_OBJECTS_OF, "targetObjectsOf");
vocab_term!(ShaclVocab, SH_TEXT, "text");
vocab_term!(ShaclVocab, SH_UNIQUE_LANG, "uniqueLang");
vocab_term!(ShaclVocab, SH_XONE, "xone");
vocab_term!(ShaclVocab, SH_SOURCE_CONSTRAINT, "sourceConstraint");

// SHACL 1.2
vocab_term!(ShaclVocab, SH_REIFICATION_REQUIRED, "reificationRequired");
vocab_term!(ShaclVocab, SH_REIFIER_SHAPE, "reifierShape");

vocab_term!(ShaclVocab, SH_CLOSED_CONSTRAINT_COMPONENT, "ClosedConstraintComponent");
vocab_term!(
    ShaclVocab,
    SH_REIFIER_SHAPE_CONSTRAINT_COMPONENT,
    "ReifierShapeConstraintComponent"
);
