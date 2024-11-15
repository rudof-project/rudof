use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompiledShaclError {
    #[error("Conversion from IriRef failed")]
    IriRefConversion,
    #[error("Could not found the shape that it was been searched")]
    ShapeNotFound,
    #[error("Could not convert to Literal")]
    LiteralConversion,
    #[error("Erro: the ID of the shape is not valid")]
    ShapeIdIsNotValid,
}
