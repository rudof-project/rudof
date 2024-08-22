use std::path::Path;

use clap::ValueEnum;
use shacl_ast::Schema;
use srdf::RDFFormat;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDFSparql;

use crate::context::ValidationContext;
use crate::shape::ShapeValidator;
use crate::store::graph::Graph;
use crate::store::sparql::Sparql;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq)]
pub enum ShaclValidationMode {
    Default,
    SPARQL,
}

pub trait Validator<S: SRDFBasic> {
    fn validation_context<'a>(&'a self, schema: &'a Schema) -> ValidationContext<'a, S>;

    fn validate(&self, schema: Schema) -> Result<ValidationReport<S>, ValidateError> {
        let validation_context = self.validation_context(&schema);
        let mut report = ValidationReport::default();

        for (_, shape) in schema.iter() {
            let shape_validator = ShapeValidator::new(shape, &validation_context, None);

            match shape_validator.validate() {
                Ok(validation_results) => {
                    report.add_results(validation_results);
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }

        Ok(report)
    }
}

pub struct GraphValidator {
    store: Graph,
    mode: ShaclValidationMode,
}

impl GraphValidator {
    pub fn new(
        data: &Path,
        data_format: RDFFormat,
        base: Option<&str>,
        mode: ShaclValidationMode,
    ) -> Result<Self, ValidateError> {
        if mode == ShaclValidationMode::SPARQL {
            return Err(ValidateError::UnsupportedMode);
        }

        Ok(GraphValidator {
            store: Graph::new(data, data_format, base)?,
            mode,
        })
    }
}

impl Validator<SRDFGraph> for GraphValidator {
    fn validation_context<'a>(&'a self, schema: &'a Schema) -> ValidationContext<'a, SRDFGraph> {
        match self.mode {
            ShaclValidationMode::Default => ValidationContext::new_default(&self.store, schema),
            ShaclValidationMode::SPARQL => todo!(),
        }
    }
}

pub struct SparqlValidator {
    store: Sparql,
    mode: ShaclValidationMode,
}

impl SparqlValidator {
    pub fn new(data: &str, mode: ShaclValidationMode) -> Result<Self, ValidateError> {
        Ok(SparqlValidator {
            store: Sparql::new(data)?,
            mode,
        })
    }
}

impl Validator<SRDFSparql> for SparqlValidator {
    fn validation_context<'a>(&'a self, schema: &'a Schema) -> ValidationContext<SRDFSparql> {
        match self.mode {
            ShaclValidationMode::Default => ValidationContext::new_default(&self.store, schema),
            ShaclValidationMode::SPARQL => ValidationContext::new_sparql(&self.store, schema),
        }
    }
}
