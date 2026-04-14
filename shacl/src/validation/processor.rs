use std::fmt::Debug;
use rudof_rdf::rdf_core::NeighsRDF;
use crate::ir::IRSchema;
use crate::validation::engine::{Engine, NativeEngine, SparqlEngine};
use crate::validation::error::ValidationError;
use crate::validation::mode::ShaclValidationMode;
use crate::validation::report::ValidationReport;
use crate::validation::validator::Validate;

/// The basic operations of the SHACL Processor.
///
/// The ShaclProcessor trait is the one in charge of applying the SHACL
/// Validation algorithm. For this, first, the validation report is initiliazed
/// to empty, and, for each shape in the schema, the target nodes are
/// selected, and then, each validator for each constraint is applied.
pub(crate) trait ShaclProcessor<S: NeighsRDF + Debug> {
    fn store(&self) -> &S;

    fn runner(mode: &ShaclValidationMode) -> Box<dyn Engine<S>>;

    /// Executes the Validation of the provided Graph, in any of the supported
    /// formats, against the shapes graph passed as an argument. As a result,
    /// the Validation Report generated from the validation process is returned.
    ///
    /// # Arguments
    ///
    /// * `shapes_graph` - A compiled SHACL shapes graph
    /// * `mode` - The validation mode to be applied during the validation process
    fn validate(&mut self, shapes_graph: &IRSchema, mode: &ShaclValidationMode) -> Result<ValidationReport, ValidationError> {
        let mut results = Vec::new();
        let mut runner = Self::runner(mode);

        runner.build_indexes(self.store())?;

        for (_, shape) in shapes_graph.iter_with_targets() {
            results.extend(
                shape.validate(self.store(), &mut (*runner), None, Some(shape), shapes_graph)?
            )
        }

        Ok(ValidationReport::new()
            .with_results(results)
            .with_prefixmap(shapes_graph.prefix_map().clone())
        )
    }
}