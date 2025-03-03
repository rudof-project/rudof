use shacl_ast::compiled::schema::CompiledSchema;
use srdf::Rdf;

use crate::engine::native::NativeEngine;
use crate::engine::sparql::SparqlEngine;
use crate::engine::Engine;
use crate::shape::Validate;
use crate::validate_error::ValidateError;
use crate::validation_report::report::ValidationReport;

pub type DefaultShaclProcessor<Q> = ShaclProcessor<Q, NativeEngine>;
pub type SparqlShaclProcessor<S> = ShaclProcessor<S, SparqlEngine>;

pub struct ShaclProcessor<R: Rdf, E: Engine<R>> {
    store: R,
    _phantom: std::marker::PhantomData<E>,
}

impl<R: Rdf, E: Engine<R>> ShaclProcessor<R, E> {
    pub fn new(store: R) -> Self {
        Self {
            store,
            _phantom: std::marker::PhantomData::<E>,
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
    ) -> Result<ValidationReport, ValidateError> {
        // we initialize the validation report to empty
        let mut validation_results = Vec::new();

        // for each shape in the schema that has at least one target
        for (_, shape) in shapes_graph.iter_with_targets() {
            let results = shape.validate::<E>(&self.store, None)?;
            validation_results.extend(results);
        }

        let report = ValidationReport::default()
            .with_results(validation_results)
            .with_prefixmap(shapes_graph.prefix_map());

        Ok(report) // return the possibly empty validation report
    }
}
