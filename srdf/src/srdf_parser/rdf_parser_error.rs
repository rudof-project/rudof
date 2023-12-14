use thiserror::Error;

#[derive(Debug, Error)]
pub enum RDFParseError {
    #[error("RDF Error: {err}")]
    SRDFError { err: String },

    #[error("Node {node} has no value for predicate {pred}")]
    NoValuesPredicate { node: String, pred: String },

    #[error("Node {node} has more than one value for predicate {pred}: {values}")]
    MoreThanOneValuePredicate {
        node: String,
        pred: String,
        values: String,
    },
}
