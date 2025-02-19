use std::fmt::Debug;

use clap::ValueEnum;
use shacl_ast::compiled::schema::CompiledSchema;
use srdf::model::rdf::Rdf;
use srdf::model::sparql::Sparql;

use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::shape::Validate;
use crate::store::Store;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;
use crate::Subsetting;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Default)]
/// Backend used for the validation.
///
/// According to the SHACL Recommendation, there exists no concrete method for
/// implementing SHACL. Thus, by choosing your preferred SHACL Validation Mode,
/// the user can select which engine is used for the validation.
pub enum ShaclValidationMode {
    /// We use a Rust native engine in an imperative manner (performance)
    #[default]
    Native,
    /// We use a  SPARQL-based engine, which is declarative
    Sparql,
}

pub struct ShaclProcessor<R: Rdf + Spaqrl> {
    store: Store<R>,
    mode: ShaclValidationMode,
    subsetting: Subsetting,
}

impl<R: Rdf + Sparql> ShaclProcessor<R> {
    pub fn new(srdf: R, mode: ShaclValidationMode, subsetting: Subsetting) -> Self {
        Self {
            store: Store::new(srdf, subsetting != Subsetting::None),
            mode,
            subsetting,
        }
    }

    /// Executes the Validation of the provided Graph, in any of the supported
    /// formats, against the shapes graph passed as an argument. As a result,
    /// the Validation Report generated from the validation process is returned.
    ///
    /// # Arguments
    ///
    /// * `shapes_graph` - A compiled SHACL shapes graph
    pub fn validate(
        &self,
        shapes_graph: &CompiledSchema<R>,
    ) -> Result<ValidationReport<R>, ValidateError> {
        // we initialize the validation report to empty
        let mut validation_results = Vec::new();

        // for each shape in the schema
        for (_, shape) in shapes_graph.iter() {
            let results = shape.validate(
                &self.store,
                match self.mode {
                    ShaclValidationMode::Native => &NativeEngine,
                    ShaclValidationMode::Sparql => &SparqlEngine,
                },
                None,
                &self.subsetting,
            )?;
            validation_results.extend(results);
        }

        Ok(ValidationReport::new()
            .with_results(validation_results)
            .with_prefixmap(shapes_graph.prefix_map())) // return the possibly empty validation report
    }
}
