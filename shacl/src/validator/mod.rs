//! SHACL validation
//! This module contains the code for SHACL validation

mod cache;
mod config;
pub mod constraints;
pub mod engine;
pub(crate) mod error;
mod index;
mod iteration;
mod mode;
pub mod nodes;
pub mod processor;
mod recursion;
pub mod report;
pub mod store;
pub mod typing;

pub use config::ShaclConfig;
pub use mode::ShaclValidationMode;
pub use recursion::RecursionSemantics;
