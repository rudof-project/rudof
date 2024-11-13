use shacl_ast::node_shape::NodeShape;
use srdf::graph::literal::Literal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Shacl2ShExError {
    #[error("Shacl2ShEx error: Feature not implemented: {msg}")]
    NotImplemented { msg: String },

    #[error("Shacl2ShEx error: Feature not implemented: {literal}")]
    RDFNode2LabelLiteral { literal: Literal },

    #[error("Not expected node shape: {node_shape:?}")]
    NotExpectedNodeShape { node_shape: Box<NodeShape> },

    #[error("Unexpected blank node in target class declaration: {bnode:?}")]
    UnexpectedBlankNodeForTargetClass { bnode: String },

    #[error("Unexpected literal in target class declaration: {literal:?}")]
    UnexpectedLiteralForTargetClass { literal: Literal },
}

impl Shacl2ShExError {
    pub fn not_implemented(msg: &str) -> Shacl2ShExError {
        Shacl2ShExError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
