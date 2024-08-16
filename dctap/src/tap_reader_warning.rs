use thiserror::Error;

#[derive(Error, Debug)]

pub enum TapReaderWarning {
    #[error("Empty property with fields found at line: {line}")]
    EmptyProperty { line: u64 },

    #[error("Extends label found: {label} without extends ID at line {line}")]
    ExtendsLabelWithoutExtendsId { label: String, line: u64 },
}
