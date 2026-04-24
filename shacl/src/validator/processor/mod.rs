#[cfg(feature = "sparql")]
mod endpoint;
mod graph;
#[cfg(feature = "sparql")]
mod rdf_data;

use crate::error::ValidationError;
use crate::ir::IRSchema;
use crate::validator::ShaclValidationMode;
use crate::validator::engine::{Engine, Validate};
use crate::validator::report::ValidationReport;
#[cfg(feature = "sparql")]
pub use endpoint::EndpointValidation;
pub use graph::GraphValidation;
#[cfg(feature = "sparql")]
pub use rdf_data::DataValidation;
use rayon::prelude::*;
use rudof_rdf::rdf_core::NeighsRDF;
use std::fmt::Debug;

/// The basic operations of the SHACL Processor.
///
/// The ShaclProcessor trait is the one in charge of applying the SHACL
/// Validation algorithm. For this, first, the validation report is initiliazed
/// to empty, and, for each shape in the schema, the target nodes are
/// selected, and then, each validator for each constraint is applied.
pub trait ShaclProcessor<S: NeighsRDF + Debug + Send + Sync> {
    fn store(&self) -> &S;

    fn runner(mode: &ShaclValidationMode) -> Box<dyn Engine<S>>;

    /// Executes the Validation of the provided Graph, in any of the supported
    /// formats, against the shapes graph passed as an argument. As a result,
    /// the Validation Report generated from the validation process is returned.
    ///
    /// Shapes are validated in parallel using topological level ordering derived
    /// from the dependency graph. Shapes within the same level have no
    /// dependency relationships and are validated concurrently, while successive
    /// levels are processed sequentially to ensure that each shape's sub-shapes
    /// are already validated (and cached) before the shape itself runs.
    ///
    /// # Arguments
    ///
    /// * `shapes_graph` - A compiled SHACL shapes graph
    /// * `mode` - The validation mode to be applied during the validation process
    fn validate(
        &mut self,
        shapes_graph: &IRSchema,
        mode: &ShaclValidationMode,
    ) -> Result<ValidationReport, ValidationError> {
        let store = self.store();

        // Build shared indexes once. Forked engines share
        // the data, avoiding redundant scans.
        let mut master_runner = Self::runner(mode);
        master_runner.build_indexes(store)?;

        // Group shapes-with-targets by topological level so that dependencies
        // are always validated before the shapes that reference them.
        let levels = shapes_graph.shapes_with_targets_by_level();

        let mut all_results = Vec::new();

        for level in levels {
            // Fork one engine per shape in this level. Each fork shares the
            // pre-built class index and cache.
            let mut forked_runners: Vec<Box<dyn Engine<S>>> =
                level.iter().map(|_| master_runner.fork()).collect();

            // Validate all shapes in the level in parallel.
            let level_results: Vec<Result<Vec<_>, ValidationError>> = forked_runners
                .par_iter_mut()
                .zip(level.par_iter())
                .map(|(runner, idx)| {
                    let shape = shapes_graph
                        .get_shape_from_idx(idx)
                        .expect("Internal error: shape index not found in schema");
                    shape.validate(store, runner.as_mut(), None, Some(shape), shapes_graph)
                })
                .collect();

            for result in level_results {
                all_results.extend(result?);
            }
        }

        let mut pm = shapes_graph.prefix_map().clone();
        if let Some(store_pm) = store.prefixmap() {
            _ = pm.merge(store_pm);
        }

        Ok(ValidationReport::new().with_results(all_results).with_prefixmap(pm))
    }
}
