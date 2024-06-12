use crate::{tap_config::TapConfig, tap_error::TapError};
use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use tracing::debug;

#[derive(Debug, Serialize, Deserialize)]
struct TapShapeId(String);

#[derive(Debug, Serialize)]
pub struct DCTap {
    version: String,
}

impl Default for DCTap {
    fn default() -> Self {
        Self::new()
    }
}

impl DCTap {
    pub fn new() -> DCTap {
        DCTap {
            version: "0.1".to_string(),
        }
    }

    pub fn read_buf(_path: &Path, _config: TapConfig) -> Result<DCTap, TapError> {
        let dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        Ok(dctap)
    }
}
