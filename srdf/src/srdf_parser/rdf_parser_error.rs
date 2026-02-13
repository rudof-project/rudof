use crate::SLiteral;
use iri_s::IriS;
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum RDFParseError {
    #[error(transparent)]
    RDFError(#[from] crate::RDFError),

    #[error("No focus node")]
    NoFocusNode,

    #[error("Expected focus node to be boolean but found: {term}")]
    ExpectedBoolean { term: String },

    #[error("Expected focus node to be a numeric literal but found: {term}")]
    ExpectedNumber { term: String },

    #[error("Expected focus node to be IRI or BlankNode but found: {term}: {error}")]
    ExpectedIriOrBlankNode { term: String, error: String },

    #[error("Error converting subject to IRI or BlankNode: {subject}")]
    SubjectToIriOrBlankNode { subject: String },

    #[error("Expected focus node to be IRI or BNode but found: {term}")]
    UnexpectedLiteral { term: String },

    #[error("Converting Term to RDFNode failed: {term}")]
    TermToRDFNodeFailed { term: String },

    #[error("Converting Subject to RDFNode failed: {subj}")]
    SubjToRDFNodeFailed { subj: String },

    #[error("Expected focus node to be integer but found: {term}")]
    ExpectedInteger { term: String },

    #[error("Error converting literal to SLiteral: {literal}")]
    LiteralToSLiteralFailed { literal: String },

    #[error("Expected focus node to be string but found: {term}")]
    ExpectedString { term: String },

    #[error("Expected IRI or Literal value but obtained blank node: {bnode}: {msg}")]
    BlankNodeNoValue { bnode: String, msg: String },

    #[error("RDF Error: {err}")]
    SRDFError { err: String },

    #[error("Node {node} has no value for predicate {pred}")]
    NoValuesPredicate { node: String, pred: String },

    #[error("Node {node} has no value for predicate {pred}. Outgoing arcs: {outgoing_arcs}")]
    NoValuesPredicateDebug {
        node: String,
        pred: String,
        outgoing_arcs: String,
    },

    #[error("Node {node} has more than one value for predicate {pred}: {value1}, {value2}")]
    MoreThanOneValuePredicate {
        node: String,
        pred: String,
        value1: String,
        value2: String,
    },

    #[error("No instances found for {object}")]
    NoInstancesOf { object: String },

    #[error("More than one instance of {object}: instance1: {value1}, instance2: {value2}")]
    MoreThanOneInstanceOf {
        object: String,
        value1: String,
        value2: String,
    },

    #[error("Expected node to act as subject: {node} in {context}")]
    ExpectedSubject { node: String, context: String },

    #[error("Error parsing RDF list. Value: {node} has already been visited")]
    RecursiveRDFList { node: String },

    #[error("Expected IRI, but found {term}")]
    ExpectedIRI { term: String },

    #[error("Expected IRI but found BNode {bnode}")]
    ExpectedIRIFoundBNode { bnode: String },

    #[error("Expected Literal, but found {term}")]
    ExpectedLiteral { term: String },

    #[error("Expected simple lliterliteral, but found {term}")]
    ExpectedSLiteral { term: String },

    #[error("Expected focus to act as subject, found {focus}")]
    ExpectedFocusAsSubject { focus: String },

    #[error("Unexpected Blank Node: {term}")]
    UnexpectedBNode { term: String },

    #[error("Expected IRI but found Literal {lit}")]
    ExpectedIRIFoundLiteral { lit: SLiteral },

    #[error("Condition {condition_name} failed for node {node}")]
    NodeDoesntSatisfyCondition { condition_name: String, node: String },

    #[error("Both branches of an OR parser failed. Error1: {err1}, Error2: {err2}")]
    FailedOr {
        err1: Box<RDFParseError>,
        err2: Box<RDFParseError>,
    },

    #[error("Not parser failed because internal parser passed with value: {value}")]
    FailedNot { value: String },

    #[error("Error obtaining subjects whose value for property {property} is {value}: {err}")]
    ErrorSubjectsPredicateObject {
        property: String,
        value: String,
        err: String,
    },

    #[error("Error parsing by type. Unknown type: {iri_type}")]
    UnknownType { iri_type: IriS },

    #[error("{msg}")]
    Custom { msg: String },

    #[error("Expected IRI for property {property} of node {focus}: {error}")]
    PropertyValueExpectedIRI {
        focus: String,
        property: IriS,
        error: String,
    },

    #[error("Expected IRI or BlankNode for property {property} of node {focus}: {error}")]
    PropertyValueExpectedIRIOrBlankNode {
        focus: String,
        property: IriS,
        error: String,
    },
}
