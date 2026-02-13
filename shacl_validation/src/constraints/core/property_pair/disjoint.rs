use crate::constraints::NativeValidator;
use crate::constraints::SparqlValidator;
use crate::constraints::constraint_error::ConstraintError;
use crate::helpers::constraint::validate_with_focus;
use crate::iteration_strategy::ValueNodeIteration;
use crate::shacl_engine::engine;
use crate::validation_report::result::ValidationResult;
use crate::value_nodes::ValueNodes;
use shacl_ir::compiled::component_ir::ComponentIR;
use shacl_ir::compiled::component_ir::Disjoint;
use shacl_ir::compiled::shape::ShapeIR;
use shacl_ir::schema_ir::SchemaIR;
use rdf::rdf_core::{NeighsRDF, Rdf, SHACLPath, query::QueryRDF, term::Triple};
use std::fmt::Debug;
use tracing::debug;

impl<R: NeighsRDF + Debug + 'static> NativeValidator<R> for Disjoint {
    fn validate_native(
        &self,
        component: &ComponentIR,
        shape: &ShapeIR,
        store: &R,
        _engine: &mut dyn engine::Engine<R>,
        value_nodes: &ValueNodes<R>,
        _source_shape: Option<&ShapeIR>,
        maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        let check = |focus: &R::Term, value_node: &R::Term| {
            let subject: R::Subject = <R as Rdf>::term_as_subject(focus).unwrap();
            let iri_owned: R::IRI = self.iri().clone().into();
            let triples_to_compare = match store
                .triples_with_subject_predicate(&subject, &iri_owned)
            {
                Ok(iter) => iter,
                Err(e) => {
                    debug!(
                        "Disjoint: Error trying to find triples for subject {} and predicate {}: {e}",
                        subject,
                        self.iri()
                    );
                    return true;
                }
            };
            for triple in triples_to_compare {
                let value = triple.obj();
                let value1 = <R as Rdf>::term_as_object(value_node).unwrap();
                let value2 = <R as Rdf>::term_as_object(value).unwrap();
                debug!("Comparing {value1} != {value2}");
                if value1 == value2 {
                    debug!(
                        "Disjoint constraint violated: {value_node} is not disjoint with {value}"
                    );
                    return true;
                }
            }
            false
        };
        let message = format!("Disjoint failed. Property {}", self.iri());

        validate_with_focus(
            component,
            shape,
            value_nodes,
            ValueNodeIteration,
            check,
            &message,
            maybe_path,
        )
    }
}

impl<S: QueryRDF + Debug + 'static> SparqlValidator<S> for Disjoint {
    fn validate_sparql(
        &self,
        _component: &ComponentIR,
        _shape: &ShapeIR,
        _store: &S,
        _value_nodes: &ValueNodes<S>,
        _source_shape: Option<&ShapeIR>,
        _maybe_path: Option<SHACLPath>,
        _shapes_graph: &SchemaIR,
    ) -> Result<Vec<ValidationResult>, ConstraintError> {
        Err(ConstraintError::NotImplemented("Disjoint".to_string()))
    }
}
