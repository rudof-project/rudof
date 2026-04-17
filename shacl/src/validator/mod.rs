//! SHACL validation
//! This module contains the code for SHACL validation

pub(crate) mod error;
mod cache;
pub mod report;
mod config;
mod mode;
pub mod store;
pub mod processor;
pub mod engine;
mod index;
pub mod nodes;
mod iteration;
pub mod constraints;

pub use config::ShaclConfig;
pub use mode::ShaclValidationMode;
