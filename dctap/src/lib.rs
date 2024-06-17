//! DCTAP Processor
//!
//! This module contains a simple [DCTAP](https://www.dublincore.org/specifications/dctap/) processor
//!
//!
//! DCTAP (Dublin Core Tabular Application Profiles) is a simple model that can be used to specify data models
//!
pub mod dctap;
pub mod tap_config;
pub mod tap_error;
pub mod tap_headers;
pub mod tap_reader;
pub mod tap_shape;
pub mod tap_statement;

pub use crate::tap_config::*;
pub use crate::tap_error::*;
pub use crate::tap_reader::*;
pub use crate::tap_shape::*;
pub use crate::tap_statement::*;
pub use dctap::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Default, Clone)]
pub struct PropertyId {
    str: String,
}

impl PropertyId {
    pub fn new(str: &str) -> PropertyId {
        PropertyId {
            str: str.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ShapeId {
    str: String,
}

impl ShapeId {
    pub fn new(str: &str) -> ShapeId {
        ShapeId {
            str: str.to_string(),
        }
    }
}

impl Default for ShapeId {
    fn default() -> Self {
        Self {
            str: "default".to_string(),
        }
    }
}
