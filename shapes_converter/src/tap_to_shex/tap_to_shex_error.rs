use thiserror::Error;

#[derive(Error, Debug)]
pub enum Tap2ShExError {
    #[error("Tap2ShEx converter, not implemented: {msg}")]
    NotImplemented { msg: String },
}

impl Tap2ShExError {
    pub fn not_implemented(msg: &str) -> Tap2ShExError {
        Tap2ShExError::NotImplemented {
            msg: msg.to_string(),
        }
    }
}
