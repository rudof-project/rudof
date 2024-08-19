use std::path::Path;
use std::sync::Arc;

use crate::context::ValidationContext;
use crate::runner::ValidatorRunner;
use crate::shape::ShapeValidator;
use crate::store::graph::Graph;
use crate::store::sparql::Sparql;
use crate::store::Store;
use crate::targets::Targets;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;
use crate::validation_report::result::LazyValidationIterator;
use clap::ValueEnum;
use shacl_ast::Schema;
use srdf::RDFFormat;
use srdf::SRDFBasic;
use srdf::SRDFGraph;
use srdf::SRDFSparql;

#[derive(ValueEnum, Copy, Clone, Debug)]
pub enum ShaclValidationMode {
    Default,
    SPARQL,
}

pub trait Validator<S: SRDFBasic, R: ValidatorRunner<S>> {
    fn validation_context(&self, schema: &Schema) -> ValidationContext<S, R>;

    fn validate(&self, schema: Schema) -> Result<ValidationReport<S>, ValidateError>
    where
        Self: Sync,
        S::Term: Send + Sync,
    {
        let validation_context = self.validation_context(&schema);
        let validation_context = Arc::new(validation_context);

        let focus_nodes = Targets::new(std::iter::empty());
        let focus_nodes = Arc::new(focus_nodes);

        // Ensure that ShapeValidator and its methods accept the appropriate lifetimes
        let results = schema.iter().filter_map(|(_, shape)| {
            let shape_validator =
                ShapeValidator::new(Arc::new(shape), Arc::clone(&validation_context));
            match shape_validator.validate(Arc::clone(&focus_nodes)) {
                Ok(validation_results) => Some(validation_results),
                Err(_) => None, // Handle errors as needed
            }
        });

        let mut report = ValidationReport::default();
        // report.add_results(LazyValidationIterator::new(results));

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

impl<R: ValidatorRunner<SRDFGraph>> Validator<SRDFGraph, R> for GraphValidator {
    fn validation_context(&self, schema: &Schema) -> ValidationContext<SRDFGraph, R> {
        // match self.mode {
        //     ShaclValidationMode::Default => {
        //         ValidationContext::new_default(self.store.store(), schema.to_owned())
        //     }
        //     ShaclValidationMode::SPARQL => todo!(),
        // }
        todo!()
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

impl<R: ValidatorRunner<SRDFSparql>> Validator<SRDFSparql, R> for SparqlValidator {
    fn validation_context(&self, schema: &Schema) -> ValidationContext<SRDFSparql, R> {
        // match self.mode {
        //     ShaclValidationMode::Default => {
        //         ValidationContext::new_default(self.store.store(), schema.to_owned())
        //     }
        //     ShaclValidationMode::SPARQL => {
        //         ValidationContext::new_query(self.store.store(), schema.to_owned())
        //     }
        // }
        todo!()
    }
}
