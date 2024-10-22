#![doc = include_str!("../README.md")]

mod constraints;
mod engine;
mod focus_nodes;
mod helpers;
pub mod shacl_config;
/// The SHACL processor implementation, used for validating a data graph against
/// a shapes graph and obtaining a Validation Report as a result.
pub mod shacl_processor;
pub mod shacl_validation_vocab;
mod shape;
/// Utilities for handling local graphs (serialized), SPARQL endpoints and SHACL
/// shapes graphs.
pub mod store;
pub mod validate_error;
/// The result of the validation process.
pub mod validation_report;
mod value_nodes;
