use crate::error::ValidationError;
use crate::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use crate::validator::cache::ValidationCache;
use crate::validator::constraints::{ShaclComponent, SparqlValidator, ValidatorDeref};
use crate::validator::engine::{Engine, select};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationResult;
use indoc::formatdoc;
use iri_s::IriS;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Term};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::fmt::Debug;

pub struct SparqlEngine {
    cache: ValidationCache,
}

impl SparqlEngine {
    pub fn new() -> Self {
        Self {
            cache: ValidationCache::new(),
        }
    }
}

impl<S: QueryRDF + NeighsRDF + Debug + 'static> Engine<S> for SparqlEngine {
    fn evaluate(
        &mut self,
        store: &S,
        shape: &IRShape,
        component: &IRComponent,
        value_nodes: &ValueNodes<S>,
        source_shape: Option<&IRShape>,
        maybe_path: Option<&SHACLPath>,
        shapes_graph: &IRSchema,
    ) -> Result<Vec<ValidationResult>, ValidationError> {
        let shacl_component = ShaclComponent::new(component);
        let validator: &dyn SparqlValidator<S> = shacl_component.deref();

        validator
            .validate_sparql(
                component,
                shape,
                store,
                value_nodes,
                source_shape,
                maybe_path,
                shapes_graph,
            )
            .map_err(|e| ValidationError::ConstraintError {
                component: component.to_string(),
                source: Box::new(e),
            })
    }

    // If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    // in SG then { t } is a target from any data graph for s in SG.
    fn target_node(&self, store: &S, node: &Object) -> Result<FocusNodes<S>, ValidationError> {
        let node: S::Term = node.clone().into();
        if node.is_blank_node() {
            return Err(ValidationError::TargetNodeBNode);
        }

        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                BIND ({} AS ?this)
            }}
        ", node};

        select(store, &query, "this").map_err(|e| ValidationError::SparqlError {
            msg: "target_node".to_string(),
            source: Box::new(e),
        })?;

        Err(ValidationError::NotImplemented {
            msg: "target_node not implemented".to_string(),
        })
    }

    fn target_class(&self, store: &S, class: &Object) -> Result<FocusNodes<S>, ValidationError> {
        let class: S::Term = class.clone().into();
        if !class.is_iri() {
            return Err(ValidationError::TargetClassNotIri);
        }

        let query = formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

            SELECT DISTINCT ?this
            WHERE {{
                ?this rdf:type/rdfs:subClassOf* {} .
            }}
        ", class};

        select(store, &query, "this").map_err(|e| ValidationError::SparqlError {
            msg: "target_class".to_string(),
            source: Box::new(e),
        })?;

        Err(ValidationError::NotImplemented {
            msg: "target_class not implemented".to_string(),
        })
    }

    fn target_subject_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, ValidationError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this {} ?any .
            }}
        ", predicate};

        select(store, &query, "this").map_err(|e| ValidationError::SparqlError {
            msg: "target_subject_of".to_string(),
            source: Box::new(e),
        })?;

        Err(ValidationError::NotImplemented {
            msg: "target_subject_of not implemented".to_string(),
        })
    }

    fn target_object_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, ValidationError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any {} ?this.
            }}
        ", predicate};

        select(store, &query, "this").map_err(|e| ValidationError::SparqlError {
            msg: "target_object_of".to_string(),
            source: Box::new(e),
        })?;

        Err(ValidationError::NotImplemented {
            msg: "target_object_of not implemented".to_string(),
        })
    }

    fn implicit_target_class(&self, _: &S, _: &Object) -> Result<FocusNodes<S>, ValidationError> {
        Err(ValidationError::NotImplemented {
            msg: "implicit_target_class not implemented".to_string(),
        })
    }

    fn record_validation(&mut self, node: Object, shape_idx: ShapeLabelIdx, results: Vec<ValidationResult>) {
        self.cache.record(node, shape_idx, results)
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.has_validated(node, shape_idx)
    }

    fn get_cached_results(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<&Vec<ValidationResult>> {
        self.cache.get_results(node, shape_idx)
    }
}

impl Default for SparqlEngine {
    fn default() -> Self {
        Self::new()
    }
}
