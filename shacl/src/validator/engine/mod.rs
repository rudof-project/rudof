pub(crate) mod error;
mod focus_nodes_ops;
mod native;
#[cfg(feature = "sparql")]
mod sparql;
mod validate;
mod value_nodes_ops;

use crate::ir::{IRComponent, IRPropertyShape, IRSchema, IRShape, ShapeLabelIdx};
use crate::types::Target;
use iri_s::IriS;
use rudof_rdf::rdf_core::term::Object;
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashSet;

#[cfg(feature = "sparql")]
use crate::error::SparqlError;
use crate::error::ValidationError;
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
pub use native::NativeEngine;
use rudof_rdf::rdf_core::query::QueryRDF;
#[cfg(feature = "sparql")]
pub use sparql::SparqlEngine;
pub use validate::Validate;

pub trait Engine<S: NeighsRDF> {
    /// Pre-builds internal indexes from the data graph for faster target resolution
    ///
    /// This should be called **once** before the validation loop starts
    fn build_indexes(&mut self, _store: &S) -> Result<(), ValidationError> {
        Ok(())
    }

    fn evaluate(
        &mut self,
        store: &S,
        shape: &IRShape,
        component: &IRComponent,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError>;

    fn focus_nodes(&self, store: &S, targets: &[Target]) -> Result<FocusNodes<S>, ValidationError> {
        let targets_iter: Vec<_> = targets
            .iter()
            .flat_map(|target| match target {
                Target::Node(n) => self.target_node(store, n),
                Target::Class(c) => self.target_class(store, c),
                Target::SubjectsOf(p) => self.target_subject_of(store, p),
                Target::ObjectsOf(p) => self.target_object_of(store, p),
                Target::ImplicitClass(n) => self.implicit_target_class(store, n),
                Target::WrongNode(_) => todo!(),
                Target::WrongClass(_) => todo!(),
                Target::WrongSubjectsOf(_) => todo!(),
                Target::WrongObjectsOf(_) => todo!(),
                Target::WrongImplicitClass(_) => todo!(),
            })
            .flatten()
            .collect();

        Ok(FocusNodes::from_iter(targets_iter))
    }

    /// If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    /// in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &Object) -> Result<FocusNodes<S>, ValidationError>;

    fn target_class(&self, store: &S, class: &Object) -> Result<FocusNodes<S>, ValidationError>;

    fn target_subject_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, ValidationError>;

    fn target_object_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, ValidationError>;

    fn implicit_target_class(&self, store: &S, shape: &Object) -> Result<FocusNodes<S>, ValidationError>;

    fn path(&self, store: &S, shape: &IRPropertyShape, focus_node: &S::Term) -> Result<FocusNodes<S>, ValidationError> {
        let nodes =
            store
                .objects_for_shacl_path(focus_node, shape.path())
                .map_err(|e| ValidationError::ObjectsShaclPath {
                    focus_node: focus_node.to_string(),
                    shacl_path: shape.path().to_string(),
                    error: e.to_string(),
                })?;

        Ok(FocusNodes::new(nodes))
    }

    fn record_validation(&mut self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>);

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool;

    /// Returns the cached validation results for a given `(node, shape_idx)` pair, if any.
    fn get_cached_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<&Vec<ValidationResult>>;
}

fn select<S: QueryRDF>(store: &S, query: &String, index: &str) -> Result<HashSet<S::Term>, SparqlError> {
    let mut out = HashSet::new();

    let query = match store.query_select(query) {
        Ok(sol) => sol,
        Err(e) => {
            return Err(SparqlError::Query {
                query: query.to_string(),
                err: e.to_string(),
            });
        },
    };

    for sol in query.iter() {
        if let Some(sol) = sol.find_solution(index) {
            out.insert(sol.to_owned());
        }
    }

    Ok(out)
}
