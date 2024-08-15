use thiserror::Error;

#[derive(Error, Debug)]

pub enum TapReaderWarning {
    #[error("Empty property with fields found at line: {line}")]
    EmptyProperty { line: u64 },
}
