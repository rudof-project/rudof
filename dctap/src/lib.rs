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
pub mod tap_shape;
pub mod tap_statement;

pub use crate::tap_config::*;
pub use crate::tap_error::*;
pub use crate::tap_error::*;
pub use crate::tap_shape::*;
pub use crate::tap_statement::*;
pub use dctap::*;
pub use dctap::*;
