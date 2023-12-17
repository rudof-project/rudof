use thiserror::Error;

use crate::literal::Literal;

#[derive(Debug, Error)]
pub enum RDFParseError {
    #[error("RDF Error: {err}")]
    SRDFError { err: String },

    #[error("Node has no value for predicate {pred}")]
    NoValuesPredicate { node: String, pred: String },

    #[error("Node has more than one value for predicate {pred}: {value1}, {value2}")]
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

    #[error("Expected node to be as subject: {node}")]
    ExpectedSubject { node: String },

    #[error("Error parsing RDF list. Value: {node} has already been visited")]
    RecursiveRDFList { node: String },

    #[error("Expected IRI but found BNode {bnode}")]
    ExpectedIRIFoundBNode { bnode: String },

    #[error("Expected IRI but found Literal {lit}")]
    ExpectedIRIFoundLiteral { lit: Literal },

    #[error("Condition {condition_name} failed for node {node}")]
    NodeDoesntSatisfyCondition {
        condition_name: String,
        node: String,
    },
}
