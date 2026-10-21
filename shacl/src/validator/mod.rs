//! SHACL validation
//! This module contains the code for SHACL validation

mod cache;
mod config;
pub mod constraints;
pub mod engine;
pub(crate) mod error;
mod fetch_strategy;
mod index;
mod iteration;
mod mode;
pub mod nodes;
#[cfg(feature = "sparql")]
pub mod prefetch;
pub mod processor;
mod recursion;
pub mod report;
pub mod store;
pub mod typing;

pub use crate::ir::RecursionSemantics;
pub use config::ShaclConfig;
pub use fetch_strategy::FetchStrategy;
pub use mode::ShaclValidationMode;
