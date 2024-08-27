use srdf::literal::Literal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Shacl2ShExError {
    #[error("Shacl2ShEx error: Feature not implemented: {msg}")]
    NotImplemented { msg: String },

    #[error("Shacl2ShEx error: Feature not implemented: {literal}")]
    RDFNode2LabelLiteral { literal: Literal },
}

impl Shacl2ShExError {
    pub fn not_implemented(msg: &str) -> Shacl2ShExError {
        Shacl2ShExError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
