use const_format::concatcp;
use iri_s::IriS;
use lazy_static::lazy_static;

pub const SH_STR: &str = "http://www.w3.org/ns/shacl#";
pub const SH_BLANKNODE_STR: &str = concatcp!(SH_STR, "BlankNode");
pub const SH_BLANK_NODE_OR_IRI_STR: &str = concatcp!(SH_STR, "BlankNodeOrIRI");
pub const SH_BLANK_NODE_OR_LITERAL_STR: &str = concatcp!(SH_STR, "BlankNodeOrLiteral");
pub const SH_INFO_STR: &str = concatcp!(SH_STR, "Info");
pub const SH_IRI_STR: &str = concatcp!(SH_STR, "IRI");
pub const SH_IRI_OR_LITERAL_STR: &str = concatcp!(SH_STR, "IRIOrLiteral");
pub const SH_LITERAL_STR: &str = concatcp!(SH_STR, "Literal");
pub const SH_NODE_SHAPE_STR: &str = concatcp!(SH_STR, "NodeShape");
pub const SH_PROPERTY_SHAPE_STR: &str = concatcp!(SH_STR, "PropertyShape");
pub const SH_SHAPE_STR: &str = concatcp!(SH_STR, "Shape");
pub const SH_SCHEMA_STR: &str = concatcp!(SH_STR, "Schema");
pub const SH_VALIDATION_REPORT_STR: &str = concatcp!(SH_STR, "ValidationReport");
pub const SH_VALIDATION_RESULT_STR: &str = concatcp!(SH_STR, "ValidationResult");
pub const SH_VIOLATION_STR: &str = concatcp!(SH_STR, "Violation");
pub const SH_WARNING_STR: &str = concatcp!(SH_STR, "Warning");
pub const SH_AND_STR: &str = concatcp!(SH_STR, "and");
pub const SH_CLASS_STR: &str = concatcp!(SH_STR, "class");
pub const SH_CLOSED_STR: &str = concatcp!(SH_STR, "closed");
pub const SH_CONFORMS_STR: &str = concatcp!(SH_STR, "conforms");
pub const SH_DATATYPE_STR: &str = concatcp!(SH_STR, "datatype");
pub const SH_DEACTIVATED_STR: &str = concatcp!(SH_STR, "deactivated");
pub const SH_DESCRIPTION_STR: &str = concatcp!(SH_STR, "description");
pub const SH_DISJOINT_STR: &str = concatcp!(SH_STR, "disjoint");
pub const SH_EQUALS_STR: &str = concatcp!(SH_STR, "equals");
pub const SH_ENTAILMENT_STR: &str = concatcp!(SH_STR, "entailment");
pub const SH_FLAGS_STR: &str = concatcp!(SH_STR, "flags");
pub const SH_FOCUS_NODE_STR: &str = concatcp!(SH_STR, "focusNode");
pub const SH_GROUP_STR: &str = concatcp!(SH_STR, "group");
pub const SH_HAS_VALUE_STR: &str = concatcp!(SH_STR, "hasValue");
pub const SH_IGNORED_PROPERTIES_STR: &str = concatcp!(SH_STR, "ignoredProperties");
pub const SH_IN_STR: &str = concatcp!(SH_STR, "in");
pub const SH_LANGUAGE_IN_STR: &str = concatcp!(SH_STR, "languageIn");
pub const SH_LESS_THAN_STR: &str = concatcp!(SH_STR, "lessThan");
pub const SH_LESS_THAN_OR_EQUALS_STR: &str = concatcp!(SH_STR, "lessThanOrEquals");
pub const SH_MIN_COUNT_STR: &str = concatcp!(SH_STR, "minCount");
pub const SH_MAX_COUNT_STR: &str = concatcp!(SH_STR, "maxCount");
pub const SH_MIN_INCLUSIVE_STR: &str = concatcp!(SH_STR, "minInclusive");
pub const SH_MIN_EXCLUSIVE_STR: &str = concatcp!(SH_STR, "minExclusive");
pub const SH_MAX_INCLUSIVE_STR: &str = concatcp!(SH_STR, "maxInclusive");
pub const SH_MAX_EXCLUSIVE_STR: &str = concatcp!(SH_STR, "maxExclusive");
pub const SH_MIN_LENGTH_STR: &str = concatcp!(SH_STR, "minLength");
pub const SH_MAX_LENGTH_STR: &str = concatcp!(SH_STR, "maxLength");
pub const SH_MESSAGE_STR: &str = concatcp!(SH_STR, "message");
pub const SH_NAME_STR: &str = concatcp!(SH_STR, "name");
pub const SH_NODE_KIND_STR: &str = concatcp!(SH_STR, "nodeKind");
pub const SH_NODE_STR: &str = concatcp!(SH_STR, "node");
pub const SH_NOT_STR: &str = concatcp!(SH_STR, "not");
pub const SH_OR_STR: &str = concatcp!(SH_STR, "or");
pub const SH_ORDER_STR: &str = concatcp!(SH_STR, "order");
pub const SH_PATH_STR: &str = concatcp!(SH_STR, "path");
pub const SH_PATTERN_STR: &str = concatcp!(SH_STR, "pattern");
pub const SH_PROPERTY_STR: &str = concatcp!(SH_STR, "property");
pub const SH_QUALIFIED_MIN_COUNT_STR: &str = concatcp!(SH_STR, "qualifiedMinCount");
pub const SH_QUALIFIED_MAX_COUNT_STR: &str = concatcp!(SH_STR, "qualifiedMaxCount");
pub const SH_QUALIFIED_VALUE_SHAPE_STR: &str = concatcp!(SH_STR, "qualifiedValueShape");
pub const SH_QUALIFIED_VALUE_SHAPES_DISJOINT_STR: &str =
    concatcp!(SH_STR, "qualifiedValueShapesDisjoint");
pub const SH_RESULT_STR: &str = concatcp!(SH_STR, "result");
pub const SH_RESULT_PATH_STR: &str = concatcp!(SH_STR, "resultPath");
pub const SH_RESULT_SEVERITY_STR: &str = concatcp!(SH_STR, "resultSeverity");
pub const SH_RESULT_MESSAGE_STR: &str = concatcp!(SH_STR, "resultMessage");
pub const SH_SHAPES_GRAPH_STR: &str = concatcp!(SH_STR, "shapesGraph");
pub const SH_SEVERITY_STR: &str = concatcp!(SH_STR, "severity");
pub const SH_SOURCE_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "sourceConstraintComponent");
pub const SH_SOURCE_SHAPE_STR: &str = concatcp!(SH_STR, "sourceShape");
pub const SH_VALUE_STR: &str = concatcp!(SH_STR, "value");
pub const SH_TARGET_NODE_STR: &str = concatcp!(SH_STR, "targetNode");
pub const SH_TARGET_CLASS_STR: &str = concatcp!(SH_STR, "targetClass");
pub const SH_TARGET_SUBJECTS_OF_STR: &str = concatcp!(SH_STR, "targetSubjectsOf");
pub const SH_TARGET_OBJECTS_OF_STR: &str = concatcp!(SH_STR, "targetObjectsOf");
pub const SH_TEXT_STR: &str = concatcp!(SH_STR, "text");
pub const SH_UNIQUE_LANG_STR: &str = concatcp!(SH_STR, "uniqueLang");
pub const SH_XONE_STR: &str = concatcp!(SH_STR, "xone");
pub const SH_SOURCE_CONSTRAINT_STR: &str = concatcp!(SH_STR, "sourceConstraint");
pub const SH_CLASS_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "ClassConstraintComponent");
pub const SH_DATATYPE_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "DatatypeConstraintComponent");
pub const SH_NODE_KIND_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "NodeKindConstraintComponent");
pub const SH_MIN_COUNT_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MinCountConstraintComponent");
pub const SH_MAX_COUNT_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MaxCountConstraintComponent");
pub const SH_MIN_EXCLUSIVE_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MinExclusiveConstraintComponent");
pub const SH_MAX_EXCLUSIVE_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MaxExclusiveConstraintComponent");
pub const SH_MIN_INCLUSIVE_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MinInclusiveConstraintComponent");
pub const SH_MAX_INCLUSIVE_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MaxInclusiveConstraintComponent");
pub const SH_MIN_LENGTH_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MinLengthConstraintComponent");
pub const SH_MAX_LENGTH_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "MaxLengthConstraintComponent");
pub const SH_PATTERN_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "PatternConstraintComponent");
pub const SH_LANGUAGE_IN_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "LanguageInConstraintComponent");
pub const SH_UNIQUE_LANG_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "UniqueLangConstraintComponent");
pub const SH_EQUALS_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "EqualsConstraintComponent");
pub const SH_DISJOINT_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "DisjointConstraintComponent");
pub const SH_HAS_VALUE_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "HasValueConstraintComponent");
pub const SH_IN_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "InConstraintComponent");
pub const SH_LESS_THAN_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "LessThanConstraintComponent");
pub const SH_LESS_THAN_OR_EQUALS_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "LessThanOrEqualsConstraintComponent");
pub const SH_NOT_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "NotConstraintComponent");
pub const SH_OR_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "OrConstraintComponent");
pub const SH_AND_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "AndConstraintComponent");
pub const SH_XONE_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "XoneConstraintComponent");
pub const SH_NODE_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "NodeConstraintComponent");
pub const SH_PROPERTY_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "PropertyShapeComponent");
pub const SH_QUALIFIED_MIN_COUNT_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "QualifiedMinCountConstraintComponent");
pub const SH_QUALIFIED_MAX_COUNT_CONSTRAINT_COMPONENT_STR: &str =
    concatcp!(SH_STR, "QualifiedMaxCountConstraintComponent");
pub const SH_CLOSED_CONSTRAINT_COMPONENT_STR: &str = concatcp!(SH_STR, "ClosedConstraintComponent");

lazy_static! {
    pub static ref SH: IriS = IriS::new_unchecked(SH_STR);
    pub static ref SH_BLANKNODE: IriS = IriS::new_unchecked(SH_BLANKNODE_STR);
    pub static ref SH_BLANK_NODE_OR_IRI: IriS = IriS::new_unchecked(SH_BLANK_NODE_OR_IRI_STR);
    pub static ref SH_BLANK_NODE_OR_LITERAL: IriS =
        IriS::new_unchecked(SH_BLANK_NODE_OR_LITERAL_STR);
    pub static ref SH_INFO: IriS = IriS::new_unchecked(SH_INFO_STR);
    pub static ref SH_IRI: IriS = IriS::new_unchecked(SH_IRI_STR);
    pub static ref SH_IRI_OR_LITERAL: IriS = IriS::new_unchecked(SH_IRI_OR_LITERAL_STR);
    pub static ref SH_LITERAL: IriS = IriS::new_unchecked(SH_LITERAL_STR);
    pub static ref SH_NODE_SHAPE: IriS = IriS::new_unchecked(SH_NODE_SHAPE_STR);
    pub static ref SH_PROPERTY_SHAPE: IriS = IriS::new_unchecked(SH_PROPERTY_SHAPE_STR);
    pub static ref SH_SHAPE: IriS = IriS::new_unchecked(SH_SHAPE_STR);
    pub static ref SH_SCHEMA: IriS = IriS::new_unchecked(SH_SCHEMA_STR);
    pub static ref SH_VALIDATION_REPORT: IriS = IriS::new_unchecked(SH_VALIDATION_REPORT_STR);
    pub static ref SH_VALIDATION_RESULT: IriS = IriS::new_unchecked(SH_VALIDATION_RESULT_STR);
    pub static ref SH_VIOLATION: IriS = IriS::new_unchecked(SH_VIOLATION_STR);
    pub static ref SH_WARNING: IriS = IriS::new_unchecked(SH_WARNING_STR);
    pub static ref SH_AND: IriS = IriS::new_unchecked(SH_AND_STR);
    pub static ref SH_CLASS: IriS = IriS::new_unchecked(SH_CLASS_STR);
    pub static ref SH_CLOSED: IriS = IriS::new_unchecked(SH_CLOSED_STR);
    pub static ref SH_CONFORMS: IriS = IriS::new_unchecked(SH_CONFORMS_STR);
    pub static ref SH_DATATYPE: IriS = IriS::new_unchecked(SH_DATATYPE_STR);
    pub static ref SH_DEACTIVATED: IriS = IriS::new_unchecked(SH_DEACTIVATED_STR);
    pub static ref SH_DESCRIPTION: IriS = IriS::new_unchecked(SH_DESCRIPTION_STR);
    pub static ref SH_DISJOINT: IriS = IriS::new_unchecked(SH_DISJOINT_STR);
    pub static ref SH_EQUALS: IriS = IriS::new_unchecked(SH_EQUALS_STR);
    pub static ref SH_ENTAILMENT: IriS = IriS::new_unchecked(SH_ENTAILMENT_STR);
    pub static ref SH_FLAGS: IriS = IriS::new_unchecked(SH_FLAGS_STR);
    pub static ref SH_FOCUS_NODE: IriS = IriS::new_unchecked(SH_FOCUS_NODE_STR);
    pub static ref SH_GROUP: IriS = IriS::new_unchecked(SH_GROUP_STR);
    pub static ref SH_HAS_VALUE: IriS = IriS::new_unchecked(SH_HAS_VALUE_STR);
    pub static ref SH_IGNORED_PROPERTIES: IriS = IriS::new_unchecked(SH_IGNORED_PROPERTIES_STR);
    pub static ref SH_IN: IriS = IriS::new_unchecked(SH_IN_STR);
    pub static ref SH_LANGUAGE_IN: IriS = IriS::new_unchecked(SH_LANGUAGE_IN_STR);
    pub static ref SH_LESS_THAN: IriS = IriS::new_unchecked(SH_LESS_THAN_STR);
    pub static ref SH_LESS_THAN_OR_EQUALS: IriS = IriS::new_unchecked(SH_LESS_THAN_OR_EQUALS_STR);
    pub static ref SH_MIN_COUNT: IriS = IriS::new_unchecked(SH_MIN_COUNT_STR);
    pub static ref SH_MAX_COUNT: IriS = IriS::new_unchecked(SH_MAX_COUNT_STR);
    pub static ref SH_MIN_INCLUSIVE: IriS = IriS::new_unchecked(SH_MIN_INCLUSIVE_STR);
    pub static ref SH_MIN_EXCLUSIVE: IriS = IriS::new_unchecked(SH_MIN_EXCLUSIVE_STR);
    pub static ref SH_MAX_INCLUSIVE: IriS = IriS::new_unchecked(SH_MAX_INCLUSIVE_STR);
    pub static ref SH_MAX_EXCLUSIVE: IriS = IriS::new_unchecked(SH_MAX_EXCLUSIVE_STR);
    pub static ref SH_MIN_LENGTH: IriS = IriS::new_unchecked(SH_MIN_LENGTH_STR);
    pub static ref SH_MAX_LENGTH: IriS = IriS::new_unchecked(SH_MAX_LENGTH_STR);
    pub static ref SH_MESSAGE: IriS = IriS::new_unchecked(SH_MESSAGE_STR);
    pub static ref SH_NAME: IriS = IriS::new_unchecked(SH_NAME_STR);
    pub static ref SH_NODE_KIND: IriS = IriS::new_unchecked(SH_NODE_KIND_STR);
    pub static ref SH_NODE: IriS = IriS::new_unchecked(SH_NODE_STR);
    pub static ref SH_NOT: IriS = IriS::new_unchecked(SH_NOT_STR);
    pub static ref SH_OR: IriS = IriS::new_unchecked(SH_OR_STR);
    pub static ref SH_ORDER: IriS = IriS::new_unchecked(SH_ORDER_STR);
    pub static ref SH_PATH: IriS = IriS::new_unchecked(SH_PATH_STR);
    pub static ref SH_PATTERN: IriS = IriS::new_unchecked(SH_PATTERN_STR);
    pub static ref SH_PROPERTY: IriS = IriS::new_unchecked(SH_PROPERTY_STR);
    pub static ref SH_QUALIFIED_MIN_COUNT: IriS = IriS::new_unchecked(SH_QUALIFIED_MIN_COUNT_STR);
    pub static ref SH_QUALIFIED_MAX_COUNT: IriS = IriS::new_unchecked(SH_QUALIFIED_MAX_COUNT_STR);
    pub static ref SH_QUALIFIED_VALUE_SHAPE: IriS =
        IriS::new_unchecked(SH_QUALIFIED_VALUE_SHAPE_STR);
    pub static ref SH_QUALIFIED_VALUE_SHAPES_DISJOINT: IriS =
        IriS::new_unchecked(SH_QUALIFIED_VALUE_SHAPES_DISJOINT_STR);
    pub static ref SH_RESULT: IriS = IriS::new_unchecked(SH_RESULT_STR);
    pub static ref SH_RESULT_PATH: IriS = IriS::new_unchecked(SH_RESULT_PATH_STR);
    pub static ref SH_RESULT_SEVERITY: IriS = IriS::new_unchecked(SH_RESULT_SEVERITY_STR);
    pub static ref SH_RESULT_MESSAGE: IriS = IriS::new_unchecked(SH_RESULT_MESSAGE_STR);
    pub static ref SH_SHAPES_GRAPH: IriS = IriS::new_unchecked(SH_SHAPES_GRAPH_STR);
    pub static ref SH_SEVERITY: IriS = IriS::new_unchecked(SH_SEVERITY_STR);
    pub static ref SH_SOURCE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_SOURCE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_SOURCE_SHAPE: IriS = IriS::new_unchecked(SH_SOURCE_SHAPE_STR);
    pub static ref SH_VALUE: IriS = IriS::new_unchecked(SH_VALUE_STR);
    pub static ref SH_TARGET_NODE: IriS = IriS::new_unchecked(SH_TARGET_NODE_STR);
    pub static ref SH_TARGET_CLASS: IriS = IriS::new_unchecked(SH_TARGET_CLASS_STR);
    pub static ref SH_TARGET_SUBJECTS_OF: IriS = IriS::new_unchecked(SH_TARGET_SUBJECTS_OF_STR);
    pub static ref SH_TARGET_OBJECTS_OF: IriS = IriS::new_unchecked(SH_TARGET_OBJECTS_OF_STR);
    pub static ref SH_TEXT: IriS = IriS::new_unchecked(SH_TEXT_STR);
    pub static ref SH_UNIQUE_LANG: IriS = IriS::new_unchecked(SH_UNIQUE_LANG_STR);
    pub static ref SH_XONE: IriS = IriS::new_unchecked(SH_XONE_STR);
    pub static ref SH_SOURCE_CONSTRAINT: IriS = IriS::new_unchecked(SH_SOURCE_CONSTRAINT_STR);
    pub static ref SH_CLASS_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_CLASS_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_DATATYPE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_DATATYPE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_NODE_KIND_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_NODE_KIND_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MIN_COUNT_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MIN_COUNT_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MAX_COUNT_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MAX_COUNT_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MIN_EXCLUSIVE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MIN_EXCLUSIVE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MAX_EXCLUSIVE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MAX_EXCLUSIVE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MIN_INCLUSIVE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MIN_INCLUSIVE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MAX_INCLUSIVE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MAX_INCLUSIVE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MIN_LENGTH_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_AND_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_MAX_LENGTH_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_MAX_LENGTH_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_PATTERN_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_PATTERN_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_LANGUAGE_IN_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_AND_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_UNIQUE_LANG_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_UNIQUE_LANG_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_EQUALS_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_EQUALS_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_DISJOINT_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_DISJOINT_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_HAS_VALUE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_HAS_VALUE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_IN_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_IN_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_LESS_THAN_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_LESS_THAN_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_LESS_THAN_OR_EQUALS_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_LESS_THAN_OR_EQUALS_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_NOT_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_NOT_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_OR_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_OR_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_AND_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_AND_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_XONE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_XONE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_NODE_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_NODE_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_PROPERTY_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_PROPERTY_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_QUALIFIED_MIN_COUNT_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_QUALIFIED_MIN_COUNT_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_QUALIFIED_MAX_COUNT_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_QUALIFIED_MAX_COUNT_CONSTRAINT_COMPONENT_STR);
    pub static ref SH_CLOSED_CONSTRAINT_COMPONENT: IriS =
        IriS::new_unchecked(SH_CLOSED_CONSTRAINT_COMPONENT_STR);
}
