//! Struct that converts DCTAP to ShEx schemas
//!
//!

use dctap::DCTap;
use shex_ast::Schema;

use crate::{Tap2ShExConfig, Tap2ShExError};
pub struct Tap2ShEx {
    config: Tap2ShExConfig,
}

impl Tap2ShEx {
    pub fn new(config: Tap2ShExConfig) -> Tap2ShEx {
        Tap2ShEx { config }
    }

    pub fn convert(&self, tap: &DCTap) -> Result<Schema, Tap2ShExError> {
        let msg = format!("Not  implemented conversion for {tap:?}");
        Err(Tap2ShExError::not_implemented(msg.as_str()))
    }
}
