use crate::error::ValidationError;
use crate::ir::{IRComponent, IRSchema, IRShape, ShapeLabelIdx};
use crate::validator::RecursionSemantics;
use crate::validator::cache::SharedTyping;
use crate::validator::constraints::{BasicSparqlValidator, ShaclComponent, ValidatorDeref, object_as_sparql};
use crate::validator::engine::{Engine, select};
use crate::validator::nodes::{FocusNodes, ValueNodes};
use crate::validator::report::ValidationOutcome;
use indoc::formatdoc;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::query::QueryRDF;
use rudof_rdf::rdf_core::term::{Object, Term};
use rudof_rdf::rdf_core::{NeighsRDF, SHACLPath};
use std::collections::HashSet;
use std::fmt::Debug;

pub struct SparqlEngine {
    cache: SharedTyping,
    /// Fixpoint semantics used to cut recursive shape references.
    recursion_semantics: RecursionSemantics,
    /// `(node, shape)` pairs currently being validated further up this
    /// engine's own call stack. Never shared across forks, unlike `cache`.
    chain: HashSet<(Object, ShapeLabelIdx)>,
}

impl SparqlEngine {
    pub fn new(recursion_semantics: RecursionSemantics) -> Self {
        Self {
            cache: SharedTyping::new(),
            recursion_semantics,
            chain: HashSet::new(),
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
    ) -> Result<ValidationOutcome, ValidationError> {
        let shacl_component = ShaclComponent::new(component);
        let validator: &dyn BasicSparqlValidator<S> = shacl_component.deref();

        validator.validate_sparql(
            component,
            shape,
            store,
            self,
            value_nodes,
            source_shape,
            maybe_path,
            shapes_graph,
        )
    }

    // If s is a shape in a shapes graph SG and s has value t for sh:targetNode
    // in SG then { t } is a target from any data graph for s in SG.
    //
    // We resolve this directly instead of issuing a SPARQL `BIND` because the latter
    // round-trips the term through the backend's literal parser, which canonicalises
    // some lexical forms (e.g. `"4.0"^^xsd:decimal` → `"4"^^xsd:decimal`) and diverges
    // from the Native engine's behaviour.
    fn target_node(&self, _: &S, node: &Object) -> Result<FocusNodes<S>, ValidationError> {
        let node: S::Term = node.clone().into();
        if node.is_blank_node() {
            return Err(ValidationError::TargetNodeBNode);
        }
        Ok(FocusNodes::single(node))
    }

    fn target_class(&self, store: &S, class: &Object) -> Result<FocusNodes<S>, ValidationError> {
        let class_sparql = object_as_sparql(class).ok_or(ValidationError::TargetClassNotIri)?;

        let query = formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

            SELECT DISTINCT ?this
            WHERE {{
                ?this rdf:type/rdfs:subClassOf* {class_sparql} .
            }}
        "};

        let results = select(store, &query, "this")?;
        Ok(FocusNodes::new(results))
    }

    fn target_subject_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, ValidationError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?this <{}> ?any .
            }}
        ", predicate};

        let results = select(store, &query, "this")?;
        Ok(FocusNodes::new(results))
    }

    fn target_object_of(&self, store: &S, predicate: &IriS) -> Result<FocusNodes<S>, ValidationError> {
        let query = formatdoc! {"
            SELECT DISTINCT ?this
            WHERE {{
                ?any <{}> ?this.
            }}
        ", predicate};

        let results = select(store, &query, "this")?;
        Ok(FocusNodes::new(results))
    }

    fn implicit_target_class(&self, store: &S, shape: &Object) -> Result<FocusNodes<S>, ValidationError> {
        let shape_sparql = object_as_sparql(shape).ok_or(ValidationError::TargetClassNotIri)?;

        let query = formatdoc! {"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

            SELECT DISTINCT ?this
            WHERE {{
                ?this rdf:type/rdfs:subClassOf* {shape_sparql} .
            }}
        "};

        let results = select(store, &query, "this")?;
        Ok(FocusNodes::new(results))
    }

    fn record_validation(&mut self, node: Object, shape_idx: ShapeLabelIdx, outcome: ValidationOutcome) {
        self.cache.record(node, shape_idx, outcome)
    }

    fn has_validated(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.cache.has_validated(node, shape_idx)
    }

    fn get_cached_outcome(&self, node: &Object, shape_idx: ShapeLabelIdx) -> Option<ValidationOutcome> {
        self.cache.get_outcome(node, shape_idx)
    }

    fn recursion_semantics(&self) -> RecursionSemantics {
        self.recursion_semantics
    }

    fn is_in_chain(&self, node: &Object, shape_idx: ShapeLabelIdx) -> bool {
        self.chain.contains(&(node.clone(), shape_idx))
    }

    fn chain_enter(&mut self, node: Object, shape_idx: ShapeLabelIdx) {
        self.chain.insert((node, shape_idx));
    }

    fn chain_exit(&mut self, node: &Object, shape_idx: ShapeLabelIdx) {
        self.chain.remove(&(node.clone(), shape_idx));
    }

    fn fork(&self) -> Box<dyn Engine<S>> {
        Box::new(SparqlEngine {
            cache: self.cache.clone(),
            recursion_semantics: self.recursion_semantics,
            chain: HashSet::new(),
        })
    }
}

impl Default for SparqlEngine {
    fn default() -> Self {
        Self::new(RecursionSemantics::default())
    }
}
