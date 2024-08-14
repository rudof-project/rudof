use std::path::Path;

use clap::ValueEnum;
use shacl_ast::shape::Shape;
use shacl_ast::Schema;
use srdf::{RDFFormat, SRDFBasic, SRDFGraph, SRDFSparql};

use crate::executor::DefaultExecutor;
use crate::executor::QueryExecutor;
use crate::executor::SHACLExecutor;
use crate::shape::Validate;
use crate::store::graph::Graph;
use crate::store::sparql::Sparql;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ShaclValidationMode {
    Default,
    SPARQL,
}

pub trait Validator<S: SRDFBasic> {
    fn executor(&self, schema: &Schema) -> Box<dyn SHACLExecutor<S> + '_>;

    fn validate(&self, schema: Schema) -> Result<ValidationReport<S>, ValidateError> {
        let results = schema
            .iter()
            .flat_map(|(_, shape)| match shape {
                Shape::NodeShape(s) => s.validate(self.executor(&schema).as_ref()),
                Shape::PropertyShape(s) => s.validate(self.executor(&schema).as_ref()),
            })
            .flatten()
            .collect::<Vec<_>>();

        let mut report = ValidationReport::default();
        report.add_results(results);

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
        Ok(GraphValidator {
            store: Graph::new(data, data_format, base)?,
            mode,
        })
    }
}

impl Validator<SRDFGraph> for GraphValidator {
    fn executor(&self, schema: &Schema) -> Box<dyn SHACLExecutor<SRDFGraph> + '_> {
        match self.mode {
            ShaclValidationMode::Default => {
                Box::new(DefaultExecutor::new(self.store.store(), schema.to_owned()))
            }
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
    fn executor(&self, schema: &Schema) -> Box<dyn SHACLExecutor<SRDFSparql> + '_> {
        match self.mode {
            ShaclValidationMode::Default => {
                Box::new(DefaultExecutor::new(self.store.store(), schema.to_owned()))
            }
            ShaclValidationMode::SPARQL => {
                Box::new(QueryExecutor::new(self.store.store(), schema.to_owned()))
            }
        }
    }
}
