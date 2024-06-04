use tracing::debug;
use std::path::PathBuf;
use crate::dctap_error::DCTapError;

#[derive(Debug)]
pub struct DCTap {
    version: String 
}

impl DCTap {
    pub fn new() -> DCTap {
        DCTap {
            version: "0.1".to_string()
        }
    }

    pub fn read_buf(path_buf: &PathBuf) -> Result<DCTap, DCTapError> {
        let dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        Ok(dctap)
    }
}