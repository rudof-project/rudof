use shex_ast::CompiledSchemaError;
use std::fmt::Debug;
use thiserror::Error;
use tokio::task::JoinError;

use crate::CardinalityError;

#[derive(Error, Debug)]
pub enum ValidationError<'a, SL>
where
    SL: Debug,
{
    #[error("ShapeLabel not found {shape_label:?} Labels: {existing_labels:?}")]
    LabelNotFoundError {
        shape_label: &'a SL,
        existing_labels: Vec<&'a SL>,
    },
    #[error("Converting Json String: {str:?}")]
    FromJsonStr { str: String, err: String },

    #[error("Compiling schema: {error:?}")]
    CompilingSchema { error: CompiledSchemaError },


    #[error("SRDF Error {error:?}")]
    SRDFError { error: String },

    #[error("Cardinality error: {ce:?}")]
    CardinalityError { ce: CardinalityError },

    #[error("JoinError: {je:?}")]
    JoinError { je: JoinError },

}

impl<'a, SL> From<JoinError> for ValidationError<'a, SL>
where
    SL: Debug,
{
    fn from(je: JoinError) -> Self {
        ValidationError::JoinError { je: je }
    }
}

impl<'a, SL> From<CompiledSchemaError> for ValidationError<'a, SL>
where
    SL: Debug,
{
    fn from(ce: CompiledSchemaError) -> Self {
        ValidationError::CompilingSchema { error: ce }
    }
}
