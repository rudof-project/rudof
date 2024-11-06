#![doc = include_str!("../README.md")]

mod constraints;
mod engine;
mod focus_nodes;
mod helpers;
/// Configuration
pub mod shacl_config;
/// The SHACL processor implementation, used for validating a data graph against
/// a shapes graph and obtaining a Validation Report as a result.
pub mod shacl_processor;
/// Vocabularies for the SHACL validation.
pub mod shacl_validation_vocab;
mod shape;
mod store;
/// Errors obtained during the validation processor.
pub mod validate_error;
/// The result of the validation process.
pub mod validation_report;
mod value_nodes;

#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq)]
pub enum Subsetting {
    None,
    Full,
    Provenance,
}
