use thiserror::Error;

#[derive(Error, Debug)]
pub enum SPARQLError {
    #[error("Query could not be performed")]
    Query { query: String, error: String },
}
