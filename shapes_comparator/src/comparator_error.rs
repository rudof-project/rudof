use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ComparatorError {
    #[error("No shape label provided")]
    NoShapeLabelProvided,
    #[error("Unknown schema format: {0}")]
    UnknownSchemaFormat(String),

    #[error("Unknown schema mode: {0}")]
    UnknownSchemaMode(String),

    #[error("Serializing to JSON: {error}")]
    JsonSerializationError { error: String },

    #[error("Shape not found for label {label}. Available shapes: {available_shapes}: {error}")]
    ShapeNotFound {
        label: String,
        available_shapes: String,
        error: String,
    },

    #[error("Not implemented feature: {feature}")]
    NotImplemented { feature: String },

    #[error("Resolving IriRef {iri_ref} failed: {error}")]
    ResolveError { iri_ref: String, error: String },

    #[error("No prefix map to dereference IriRef {iri_ref}")]
    NoPrefixMapDerefrencingIriRef { iri_ref: String },
}
