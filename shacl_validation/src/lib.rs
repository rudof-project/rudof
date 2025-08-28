#![doc = include_str!("../README.md")]

pub mod constraints;
pub mod engine;
pub mod focus_nodes;
mod helpers;
pub mod shacl_config;
/// The SHACL processor implementation, used for validating a data graph against
/// a shapes graph and obtaining a Validation Report as a result.
pub mod shacl_processor;
pub mod shacl_validation_vocab;
pub mod shape_validation;
/// Utilities for handling local graphs (serialized), SPARQL endpoints and SHACL
/// shapes graphs.
pub mod store;
pub mod validate_error;
/// The result of the validation process.
pub mod validation_report;
pub mod value_nodes;
