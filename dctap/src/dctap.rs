use crate::dctap_error::DCTapError;
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct DCTap {
    version: String,
}

impl DCTap {
    pub fn new() -> DCTap {
        DCTap {
            version: "0.1".to_string(),
        }
    }

    pub fn read_buf(path_buf: &PathBuf) -> Result<DCTap, DCTapError> {
        let dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        Ok(dctap)
    }
}
