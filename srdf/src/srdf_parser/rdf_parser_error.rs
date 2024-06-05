use iri_s::IriS;
use thiserror::Error;

use crate::literal::Literal;

#[derive(Debug, Error, PartialEq)]
pub enum RDFParseError {
    #[error("No focus node")]
    NoFocusNode,

    #[error("Expected focus node to be boolean but found: {term}")]
    ExpectedBoolean { term: String },

    #[error("Expected focus node to be integer but found: {term}")]
    ExpectedInteger { term: String },

    #[error("Expected focus node to be string but found: {term}")]
    ExpectedString { term: String },

    #[error("Expected IRI or Literal value but obtained blank node: {bnode}")]
    BlankNodeNoValue { bnode: String },

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

    #[error("Expected node to act as subject: {node}")]
    ExpectedSubject { node: String },

    #[error("Error parsing RDF list. Value: {node} has already been visited")]
    RecursiveRDFList { node: String },

    #[error("Expected IRI, but found {term}")]
    ExpectedIRI { term: String },

    #[error("Expected IRI but found BNode {bnode}")]
    ExpectedIRIFoundBNode { bnode: String },

    #[error("Expected Literal, but found {term}")]
    ExpectedLiteral { term: String },

    #[error("Expected focus to act as subject, found {focus}")]
    ExpectedFocusAsSubject { focus: String },

    #[error("Unexpected Blank Node: {term}")]
    UnexpectedBNode { term: String },

    #[error("Expected IRI but found Literal {lit}")]
    ExpectedIRIFoundLiteral { lit: Literal },

    #[error("Condition {condition_name} failed for node {node}")]
    NodeDoesntSatisfyCondition {
        condition_name: String,
        node: String,
    },

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
}
