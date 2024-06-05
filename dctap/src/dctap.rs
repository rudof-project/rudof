use serde_derive::{Deserialize, Serialize};
use tracing::debug;
use std::{collections::HashMap, path::PathBuf};
use crate::{tap_error::TapError, TapShape};

#[derive(Debug, Serialize, Deserialize)]
struct TapShapeId(String); 

#[derive(Debug, Serialize)]
pub struct DCTap {
    version: String, 
    shapes: HashMap<TapShapeId, TapShape>
}

impl DCTap {
    pub fn new() -> DCTap {
        DCTap {
            version: "0.1".to_string(),
            shapes: HashMap::new()

        }
    }

    pub fn read_buf(path_buf: &PathBuf, config: TapConfig) -> Result<DCTap, TapError> {
        let dctap = DCTap::new();
        debug!("DCTap parsed: {:?}", dctap);
        Ok(dctap)
    }
}