use std::path::Path;

use clap::ValueEnum;
use shacl_ast::compiled::schema::Schema;
use srdf::RDFFormat;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDFSparql;

use crate::shape::ShapeValidation;
use crate::store::graph::Graph;
use crate::store::sparql::Endpoint;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
/// According to the SHACL Recommendation, there exists no concrete method for
/// implementing SHACL. Thus, by choosing your preferred SHACL Validation Mode,
/// the user can select which engine is used for the validation.
pub enum ShaclValidationMode {
    /// We use a Rust native engine in an imperative manner
    Default,
    /// We use a  SPARQL-based engine, which is declarative
    SPARQL,
}

/// The Validation trait is the one in charge of applying the SHACL Validation
/// algorithm. For this, first, the validation report is initiliazed to empty,
/// and, for each shape in the schema, the target nodes are selected, and then,
/// each validator for each constraint is applied.
pub trait Validation<S: SRDFBasic> {
    fn validate(&self, schema: Schema<S>) -> Result<ValidationReport<S>, ValidateError> {
        // we initialize the validation report to empty
        let mut validation_report = ValidationReport::default();

        // for each shape in the schema
        for (_, shape) in schema.iter() {
            let shape_validator = ShapeValidation::new(shape, None);
            validation_report.add_results(shape_validator.validate()?);
        }

        Ok(validation_report) // return the possibly empty validation report
    }
}

pub struct GraphValidation {
    store: Graph,
    mode: ShaclValidationMode,
}

impl GraphValidation {
    pub fn new(
        data: &Path,
        data_format: RDFFormat,
        base: Option<&str>,
        mode: ShaclValidationMode,
    ) -> Result<Self, ValidateError> {
        if mode == ShaclValidationMode::SPARQL {
            return Err(ValidateError::UnsupportedMode);
        }

        Ok(GraphValidation {
            store: Graph::new(data, data_format, base)?,
            mode,
        })
    }
}

pub struct EndpointValidation {
    store: Endpoint,
    mode: ShaclValidationMode,
}

impl EndpointValidation {
    pub fn new(data: &str, mode: ShaclValidationMode) -> Result<Self, ValidateError> {
        Ok(EndpointValidation {
            store: Endpoint::new(data)?,
            mode,
        })
    }
}
